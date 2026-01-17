<script setup lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { v7 as uuidv7 } from "uuid";
    import { computed, onMounted, ref, watch } from "vue";
    import { AccountCard, BankCard, Player } from "@/components";
    import { ITauriTypes } from "@/types";
    import { McMsa, McUuid, TauriHttpServer, TauriTOML, useAccountStore } from "@/modules";
    import { MsaLoginResult } from "@/types/minecraft/Msa";
    import { expiresInToUnix } from "@/utils";
    import { useTheme } from "@/composables";
    import { useI18n } from "vue-i18n";

    const { locale } = useI18n();
    const { matchTheme } = useTheme();

    // ================ 账户模块 ================

    const ProfileConfig = ref<ITauriTypes.TOML.ProfileConfig>();
    const Profiles = ref<ITauriTypes.TOML.ProfileConfig["profile"]>([]);
    const AccountStore = useAccountStore();

    function reorderPicked<T extends { picked?: boolean }>(arr: T[]): T[] {
        const notPicked: T[] = [];
        const picked: T[] = [];

        for (const item of arr) {
            if (item.picked) picked.push(item);
            else notPicked.push(item);
        }

        // return [...notPicked, ...picked];
        return [...picked, ...notPicked];
    }

    // ================== 弹窗 ==================

    // 全局变量
    const createModal = ref<HTMLDialogElement>();
    const step = ref<number>(1);
    const type = ref<ITauriTypes.TOML.ProfileType>();
    const $env = import.meta.env;

    // 微软档案相关
    const msaUiText = ref<string>("Main.r/Profile.Modal.Step2.Msa.Waiting");
    const msaFailure = ref<boolean>(false);
    const msaFailureCode = ref<number>(0);
    const msaFailureText = ref<string>("");
    async function setupMsaService() {
        const LOGIN_URI =
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?" +
            new URLSearchParams({
                client_id: $env.OAUTH_MS_CLIENT_ID,
                response_type: "code",
                redirect_uri: $env.OAUTH_REDIRECT_URI_BASE + $env.OAUTH_MS_REDIRECT_URI,
                response_mode: "query",
                scope: "XboxLive.signin offline_access",
            }).toString();
        console.tInfo({ category: "MSA Login", message: "开始微软登录流程" });
        const res = await TauriHttpServer.start(36993, locale.value);
        if (res.message === "HTTP Server already running") {
            TauriHttpServer.unlistenOAuthCode();
            await TauriHttpServer.stop();
            await TauriHttpServer.start(36993, locale.value);
        }
        await TauriHttpServer.listenOAuthCode((payload) => handleMsaCodeReceived(payload));
        await openUrl(LOGIN_URI);
    }
    async function handleMsaCodeReceived(payload: ITauriTypes.HTTP.OAuthCodeReceivedPayload) {
        if (payload.path !== "/oauth/callback") return;
        if (!payload.query) return;
        const query = new URLSearchParams(payload.query);
        const code = query.get("code");
        if (!code) return;

        // 关闭监听事件
        console.tInfo({ category: "MSA Login", message: "收到微软登录授权码" });

        let msaLoginResult: MsaLoginResult;
        try {
            async function updateMsaUiText(step: number) {
                msaUiText.value = `Main.r/Profile.Modal.Step2.Msa.Process.Step${step}.Processing`;
                console.tInfo({ category: "MSA Login", message: `微软登录流程 Step ${step}/6` });
            }
            msaLoginResult = await McMsa.loginFromCode(code, updateMsaUiText);
        } catch (error) {
            const step = (error as McMsa.MsaLoginError).step;
            msaUiText.value = `Main.r/Profile.Modal.Step2.Msa.Process.Step${step}.Error`;
            msaFailure.value = true;
            msaFailureCode.value = (error as McMsa.MsaLoginError).code;
            msaFailureText.value = (error as McMsa.MsaLoginError).message;
            // console.group("MSA Login Error");
            console.tError({ category: "MSA Login", message: `微软登录流程 Step ${step}/6 失败` });
            console.tError({ category: "MSA Login", message: `  ↪ 错误码: ${msaFailureCode.value}` });
            console.tError({ category: "MSA Login", message: `  ↪ 错误信息: ${msaFailureText.value}` });
            // console.groupEnd();
            return;
        }

        const _ProfileConfig = await TauriTOML.getProfileConfig();
        const _Profiles = _ProfileConfig.profile || [];
        const _ProfileMatch = _Profiles.find(
            (profile) => profile.type === "msa" && profile.name === msaLoginResult.name && profile.uuid === msaLoginResult.uuid
        );
        if (_ProfileMatch) {
            _ProfileMatch.picked = true;
        } else {
            const access_token = await TauriTOML.encryptString(msaLoginResult.msaAccessToken);
            const refresh_token = await TauriTOML.encryptString(msaLoginResult.msaRefreshToken);
            _Profiles.push({
                guid: uuidv7(),
                type: "msa",
                name: msaLoginResult.name,
                uuid: msaLoginResult.uuid,
                picked: true,
                access_token,
                refresh_token,
                msa_expires_at: expiresInToUnix(msaLoginResult.msaExpiresIn * 1000),
                mc_expires_at: expiresInToUnix(msaLoginResult.mcExpiresIn * 1000),
                skin_info: JSON.stringify(msaLoginResult.skins),
                cape_info: JSON.stringify(msaLoginResult.capes),
            });
        }
        _Profiles.forEach((profile) => {
            if (profile.uuid !== msaLoginResult.uuid || profile.type !== "msa" || profile.name !== msaLoginResult.name) {
                profile.picked = false;
            }
        });
        AccountStore.setAccountState(msaLoginResult.name, "msa");
        _ProfileConfig.profile = _Profiles;
        await TauriTOML.saveProfileConfig(_ProfileConfig);
        ProfileConfig.value = _ProfileConfig;
        Profiles.value = _Profiles;
        createModal.value?.close();
        setTimeout(cleanup, 50) // 延迟清理以避免 UI 跳动
        console.info({ category: "MSA Login", message: "微软登录流程完成" });
    }

    // 离线档案相关
    const offlineUsername = ref<string>();
    const offlineUuidMode = ref<string>("");
    const offlineUuid = ref<string>();
    const offlineValid = computed(() => {
        if (
            !offlineUsername.value ||
            offlineUsername.value.length < 3 ||
            offlineUsername.value.length > 16 ||
            !/^[a-zA-Z0-9_]+$/.test(offlineUsername.value)
        ) {
            return false;
        }

        if (offlineUuidMode.value === "") {
            return false;
        }

        if (offlineUuidMode.value === "custom") {
            return (
                offlineUuid.value?.length === 36 &&
                /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/.test(offlineUuid.value)
            );
        }

        return true;
    });
    async function createLegacyProfile() {
        if (!offlineUsername.value || offlineUuidMode.value === "" || (offlineUuidMode.value === "custom" && !offlineUuid.value)) {
            return;
        }
        const uuid = offlineUuidMode.value === "custom" ? offlineUuid.value! : McUuid.createOfflineUUID(offlineUsername.value).dash;

        const _ProfileConfig = await TauriTOML.getProfileConfig();
        const _Profiles = _ProfileConfig.profile || [];
        const encryptedEmptyString = await TauriTOML.encryptString("None");
        _Profiles.push({
            guid: uuidv7(),
            type: "legacy",
            name: offlineUsername.value,
            uuid,
            picked: true,
            // 保留字段
            access_token: encryptedEmptyString,
            refresh_token: encryptedEmptyString,
        });
        _Profiles.forEach((profile) => {
            if (profile.uuid !== uuid) {
                profile.picked = false;
            }
        });
        AccountStore.setAccountState(offlineUsername.value, "legacy");
        await TauriTOML.saveProfileConfig(_ProfileConfig);
        Profiles.value = _Profiles;
        cleanup();
        createModal.value?.close();
    }

    // 弹窗相关
    const openCreateModal = () => {
        createModal.value?.showModal();
    };

    const cleanup = async () => {
        step.value = 1;
        type.value = undefined;
        msaFailure.value = false;
        msaFailureCode.value = 0;
        msaFailureText.value = "";
        offlineUsername.value = undefined;
        offlineUuidMode.value = "";
        offlineUuid.value = undefined;
        try {
            TauriHttpServer.unlistenOAuthCode();
            await TauriHttpServer.stop();
        } catch (e) {
            console.tWarn({ category: "Profile", message: `清理 HTTP 服务器失败: ${e}` });
        }
    };

    // 选择界面
    const topCardIndex = ref<number>(0);
    function onSwitchProfile() {
        // 先切换 index，保持 0-based 循环
        topCardIndex.value = (topCardIndex.value + 1) % Profiles.value.length;

        // 获取当前选中的 profile
        const newProfile = Profiles.value[topCardIndex.value];
        if (!newProfile) return;

        // 更新档案状态
        AccountStore.setAccountState(newProfile.name, newProfile.type);

        // 异步更新配置，但不要改变 Profiles.value 的顺序
        (async () => {
            const _ProfileConfig = await TauriTOML.getProfileConfig();

            // 标记 picked
            _ProfileConfig.profile.forEach((profile) => {
                profile.picked = profile.guid === newProfile.guid;
            });

            await TauriTOML.saveProfileConfig(_ProfileConfig);

            // 更新 ProfileConfig，但不要替换 Profiles.value
            ProfileConfig.value = _ProfileConfig;
        })();
    }
    watch(topCardIndex, async (newId, oldId) => {
        if (newId === oldId) {
            return;
        }
        const newProfile = Profiles.value[newId];
        if (newProfile) {
            const AccountName = newProfile.name;
            const AccountType = newProfile.type;
            AccountStore.setAccountState(AccountName, AccountType);
            const _ProfileConfig = await TauriTOML.getProfileConfig();
            _ProfileConfig.profile.forEach((profile) => {
                if (profile.guid === newProfile.guid) {
                    profile.picked = true;
                } else {
                    profile.picked = false;
                }
            });
            await TauriTOML.saveProfileConfig(_ProfileConfig);
            ProfileConfig.value = _ProfileConfig;
            Profiles.value = _ProfileConfig.profile || [];
        }
    });

    // 注销、删除档案
    const removeModal = ref<HTMLDialogElement>();
    function onRemoveProfile() {
        removeModal.value?.show();
    }
    async function removeCurrentProfile() {
        const _ProfileConfig = await TauriTOML.getProfileConfig();
        const _Profiles = _ProfileConfig.profile ?? [];
        const currentIndex = _Profiles.findIndex((p) => p.picked);
        if (_Profiles.length === 0 || currentIndex === -1) return;
        _Profiles.splice(currentIndex, 1);
        if (_Profiles.length > 0) {
            const nextIndex = Math.min(currentIndex, _Profiles.length - 1);
            _Profiles.forEach((p) => (p.picked = false));
            _Profiles[nextIndex].picked = true;
            AccountStore.setAccountState(_Profiles[nextIndex].name, _Profiles[nextIndex].type);
        }
        _ProfileConfig.profile = _Profiles;
        await TauriTOML.saveProfileConfig(_ProfileConfig);
        ProfileConfig.value = _ProfileConfig;
        Profiles.value = _Profiles;
    }

    // 玩家模型
    interface SkinInfo {
        id: string;
        state: "INACTIVE" | "ACTIVE";
        textureKey: string;
        url: string;
        variant: "WIDE" | "SLIM";
    }
    interface CapeInfo {
        alias: string;
        id: string;
        state: "INACTIVE" | "ACTIVE";
        url: string;
    }

    const CurrentProfile = computed(() => {
        return Profiles.value.find((i) => i.picked);
    });
    const CurrentProfileName = computed(() => {
        return CurrentProfile.value?.name;
    });
    const CurrentProfileType = computed(() => {
        return CurrentProfile.value?.type;
    });
    const CurrentProfileSkin = computed(() => {
        if (!CurrentProfile.value) return undefined;
        switch (CurrentProfile.value.type) {
            case "msa":
                const skin_info: SkinInfo[] = JSON.parse(CurrentProfile.value.skin_info || "[]");
                const active_skin = skin_info.find((i) => i.state === "ACTIVE");
                if (!active_skin) return [];
                return [active_skin.url, active_skin.variant];
            default:
                return undefined;
        }
    });
    const CurrentProfileCape = computed(() => {
        if (!CurrentProfile.value) return undefined;
        switch (CurrentProfile.value.type) {
            case "msa":
                const cape_info: CapeInfo[] = JSON.parse(CurrentProfile.value.cape_info || "[]");
                const active_cape = cape_info.find((i) => i.state === "ACTIVE");
                if (!active_cape) return undefined;
                return active_cape.url;
            default:
                return undefined;
        }
    });

    // 跳转函数
    function onCreateNewProfile(startType: "msa" | "legacy" | "yggdrasil") {
        switch (startType) {
            case "msa":
                type.value = "msa";
                step.value = 2;
                setupMsaService();
                break;
            case "legacy":
                if (Profiles.value.some((i) => i.type === "msa")) {
                    type.value = "legacy";
                    step.value = 2;
                } else {
                    return;
                }
                break;
            case "yggdrasil":
                // 暂时还没做好，先不让跳转
                return;
                if (Profiles.value.some((i) => i.type === "msa")) {
                    type.value = "yggdrasil";
                    step.value = 2;
                    break;
                } else {
                    return;
                }
        }
    }

    // ======== 钩子 ==========
    onMounted(async () => {
        ProfileConfig.value = await TauriTOML.getProfileConfig();
        Profiles.value = ProfileConfig.value?.profile || [];
        topCardIndex.value = Profiles.value.findIndex((i) => i.picked);
    });
</script>

<template>
    <div class="w-full h-[calc(100vh-128px-64px)] p-6">
        <div class="card w-full bg-base-100">
            <div class="card-body p-2! flex flex-row">
                <button class="btn btn-sm btn-ghost" @click="openCreateModal">
                    <i class="icon-[material-symbols--add-circle-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Profile.TopButtons.AddProfile") }}</span>
                </button>
                <button class="btn btn-sm btn-ghost" disabled>
                    <i class="icon-[material-symbols--drive-file-move-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Profile.TopButtons.ImportProfile") }}</span>
                </button>
                <button class="btn btn-sm btn-ghost" disabled>
                    <i class="icon-[material-symbols--drive-file-move-rtl-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Profile.TopButtons.ExportProfile") }}</span>
                </button>
                <button
                    class="btn btn-sm btn-ghost ml-auto"
                    @click="openUrl('https://www.xbox.com/games/store/minecraft-java-bedrock-edition-for-pc/9nxp44l49shj')">
                    <i class="icon-[material-symbols--shopping-cart-checkout-outline-rounded] size-5 -mx-1"></i>
                </button>
            </div>
        </div>

        <div class="flex-1 mt-4">
            <BankCard
                class="w-full! h-[calc(100%-62px)]! aspect-auto! mt-4"
                :style="{ '--bgc-perc': matchTheme('dark') ? '1.03%' : '0.75%' }"
                v-if="Profiles.length === 0">
                <div class="w-full h-full flex flex-col gap-4 justify-center items-center">
                    <i class="icon-[material-symbols--frame-exclamation-rounded] size-22"></i>
                    <span class="text-xl">{{ $t("Main.r/Profile.Profiles.NoProfile") }}</span>
                </div>
            </BankCard>
            <div class="grid grid-cols-[410px_1fr] grid-rows-[1fr] gap-4 h-full" v-else>
                <section class="h-full">
                    <div class="stack w-101.5 h-57.5">
                        <AccountCard
                            :style="{ '--bgc-perc': matchTheme('dark') ? '4.23%' : '5.79%' }"
                            v-for="card in reorderPicked(Profiles)"
                            :key="card.guid"
                            :profile="card" />
                    </div>
                    <div class="card w-full bg-base-100 mt-2">
                        <div class="card-body">
                            <h2 class="card-title">{{ $t("Main.r/Profile.ActionButtons.__Title__") }}</h2>
                            <button class="btn btn-soft btn-primary mt-2 w-full" @click="onSwitchProfile" :disabled="Profiles.length == 1">
                                <span class="text-sm">{{ $t("Main.r/Profile.ActionButtons.SwitchProfileNext") }}</span>
                            </button>
                            <button class="btn btn-soft btn-error mt-2 w-full" @click="onRemoveProfile" :disabled="Profiles.length == 1">
                                <span class="text-sm">
                                    {{
                                        AccountStore.AccountType === "legacy"
                                            ? $t("Main.r/Profile.ActionButtons.Remove")
                                            : $t("Main.r/Profile.ActionButtons.Logout")
                                    }}
                                </span>
                            </button>
                        </div>
                    </div>
                </section>
                <section class="h-full min-h-0">
                    <Player
                        v-if="CurrentProfileName"
                        :skin-url="CurrentProfileSkin?.[0]"
                        :type="CurrentProfileSkin?.[1].toLowerCase() || 'slim'"
                        :cape-url="CurrentProfileCape" />
                </section>
            </div>
        </div>

        <dialog ref="createModal" class="modal">
            <div class="modal-box">
                <!-- Step 1 -->
                <section class="flex flex-col items-center gap-2" v-if="step === 1">
                    <h3 class="text-xl font-bold">{{ $t("Main.r/Profile.Modal.Add.Title") }}</h3>
                    <p class="text-sm opacity-50">{{ $t("Main.r/Profile.Modal.Add.Description") }}</p>
                    <div class="divider w-76 mx-auto my-0"></div>
                    <button class="btn bg-[#2F2F2F] text-white border-black w-66" @click="onCreateNewProfile('msa')">
                        <svg aria-label="Microsoft logo" width="24" height="24" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                            <path d="M96 96H247V247H96" fill="#f24f23"></path>
                            <path d="M265 96V247H416V96" fill="#7eba03"></path>
                            <path d="M96 265H247V416H96" fill="#3ca4ef"></path>
                            <path d="M265 265H416V416H265" fill="#f9ba00"></path>
                        </svg>
                        <span class="text-sm">{{ $t("Main.r/Profile.Modal.Add.MsLogin") }}</span>
                    </button>
                    <button class="btn w-66" @click="onCreateNewProfile('legacy')" v-if="Profiles.some((i) => i.type === 'msa')">
                        <i class="icon-[material-symbols--safety-check-off-outline-rounded] size-6 mr-1"></i>
                        <span class="text-sm">{{ $t("Main.r/Profile.Modal.Add.Offline") }}</span>
                    </button>
                    <button class="btn w-66" @click="onCreateNewProfile('yggdrasil')" v-if="Profiles.some((i) => i.type === 'msa')" disabled>
                        <i class="icon-[material-symbols--assured-workload-outline-rounded] size-6 mr-1"></i>
                        <span class="text-sm">{{ $t("Main.r/Profile.Modal.Add.TrdParty") }}</span>
                    </button>
                    <div class="divider w-76 mx-auto my-0"></div>
                    <form method="dialog" class="w-66">
                        <button class="btn w-full" @click="cleanup">{{ $t("Main.r/Profile.Modal.Add.Cancel") }}</button>
                    </form>
                </section>
                <!-- Step 2 -->
                <section v-if="step === 2">
                    <!-- MSA -->
                    <section class="flex flex-col items-center gap-2" v-if="type === 'msa'">
                        <h3 class="text-xl font-bold">{{ $t("Main.r/Profile.Modal.Add.MsLogin") }}</h3>
                        <div class="divider w-96 mx-auto my-0"></div>
                        <span class="loading loading-dots loading-xl" v-if="!msaFailure"></span>
                        <i class="icon-[material-symbols--frame-exclamation-rounded] text-error size-16" v-else></i>
                        <p class="text-sm" :class="{ 'text-error': msaFailure }">
                            {{ $t(msaUiText) }}<span v-if="msaFailure">{{ $t("Main.r/Profile.Modal.Step2.Msa.Details") }}</span>
                        </p>
                        <p class="text-sm text-error text-center select-text" v-if="msaFailure">
                            {{ $t(msaFailureText) }}
                        </p>
                        <div class="divider w-96 mx-auto my-0"></div>
                        <form method="dialog" class="w-66">
                            <button class="btn w-full" @click="cleanup">
                                {{ $t("Main.r/Profile.Modal.Add.Cancel") }} / {{ $t("Main.r/Profile.Modal.Add.Back") }}
                            </button>
                        </form>
                    </section>
                    <!-- Legacy -->
                    <section class="flex flex-col items-center gap-2" v-else-if="type === 'legacy'">
                        <h3 class="text-xl font-bold">{{ $t("Main.r/Profile.Modal.Add.Offline") }}</h3>
                        <div class="divider w-96 mx-auto my-0"></div>
                        <input
                            type="text"
                            class="input outline-none validator"
                            :placeholder="$t('Main.r/Profile.Modal.Step2.Offline.Username')"
                            pattern="[a-zA-Z][a-zA-Z0-9_]{2,15}"
                            v-model="offlineUsername" />
                        <select class="select outline-none" v-model="offlineUuidMode">
                            <option disabled selected value="">{{ $t("Main.r/Profile.Modal.Step2.Offline.Uuid.Select.PickOne") }}</option>
                            <option value="standrad">{{ $t("Main.r/Profile.Modal.Step2.Offline.Uuid.Select.Standrad") }}</option>
                            <option value="custom">{{ $t("Main.r/Profile.Modal.Step2.Offline.Uuid.Select.Custom") }}</option>
                        </select>
                        <input
                            type="text"
                            class="input outline-none validator"
                            :placeholder="$t('Main.r/Profile.Modal.Step2.Offline.Uuid.Input')"
                            v-if="offlineUuidMode === 'custom'"
                            pattern="[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}"
                            v-model="offlineUuid" />
                        <div class="divider w-96 mx-auto my-0"></div>
                        <section class="w-76 grid grid-cols-2 gap-4">
                            <button class="btn btn-success w-full" @click="createLegacyProfile" :disabled="!offlineValid">
                                {{ $t("Main.r/Profile.Modal.Add.Create") }}
                            </button>
                            <button class="btn btn-soft w-full" @click="cleanup">{{ $t("Main.r/Profile.Modal.Add.Back") }}</button>
                        </section>
                    </section>
                    <!-- Yggdrasil -->
                    <section v-else-if="type === 'yggdrasil'"></section>
                </section>
            </div>
        </dialog>
        <dialog ref="removeModal" class="modal">
            <div class="modal-box">
                <section class="flex flex-col items-center gap-2">
                    <h1 class="text-xl font-semibold">
                        {{ $t("Main.r/Profile.Modal.Remove.Title") }}
                    </h1>
                    <div class="divider w-96 mx-auto my-0"></div>
                    <p>{{ $t("Main.r/Profile.Modal.Remove.Content.Line1") }}</p>
                    <p class="mt-4">
                        {{
                            $t("Main.r/Profile.Modal.Remove.Content.Line2", {
                                name: CurrentProfileName,
                                type: CurrentProfileType ? $t(`Aside.AccountType.${CurrentProfileType}`) : "",
                            })
                        }}
                    </p>
                    <p>UUID {{ Profiles.filter((i) => i.picked)[0]?.uuid }}</p>
                    <div class="divider w-96 mx-auto my-0"></div>
                    <form method="dialog" class="w-66 grid grid-cols-5 gap-4">
                        <button class="btn w-full btn-error col-span-3" @click="removeCurrentProfile">
                            {{ $t("Main.r/Profile.Modal.Remove.Perform") }}
                        </button>
                        <button class="btn w-full col-span-2">{{ $t("Main.r/Profile.Modal.Remove.Cancel") }}</button>
                    </form>
                </section>
            </div>
        </dialog>
    </div>
</template>
