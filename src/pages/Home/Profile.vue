<script setup lang="ts">
    import { ref, computed, onMounted } from "vue";
    import { BankCard, AccountCard, Player } from "@/components";
    import { useProfileManager } from "@/composables";
    import { useTheme } from "@/composables";
    import { useI18n } from "vue-i18n";
    import { openUrl } from "@tauri-apps/plugin-opener";

    // 国际化、主题
    const { locale } = useI18n();
    const { matchTheme } = useTheme();

    // -------------------- Profile 管理 --------------------
    const profileManager = useProfileManager();

    // 弹窗状态
    const createModal = ref<HTMLDialogElement>();
    const removeModal = ref<HTMLDialogElement>();
    const step = ref(1);
    const type = ref<"msa" | "legacy" | "yggdrasil">();

    // Legacy 输入
    const offlineUsername = ref("");
    const offlineUuidMode = ref<"standard" | "custom" | "">("");
    const offlineUuid = ref("");

    // 计算可用性
    const offlineValid = computed(() => {
        if (!offlineUsername.value || offlineUsername.value.length < 3 || offlineUsername.value.length > 16) return false;
        if (offlineUuidMode.value === "") return false;
        if (offlineUuidMode.value === "custom") {
            return offlineUuid.value?.length === 36 && /^[0-9a-fA-F\-]{36}$/.test(offlineUuid.value);
        }
        return true;
    });

    // -------------------- Computed --------------------
    const Profiles = computed(() => profileManager.profiles.value);
    const CurrentProfile = computed(() => profileManager.currentProfile.value);

    const CurrentProfileSkin = computed(() => {
        if (!CurrentProfile.value || CurrentProfile.value.Type !== "msa") return undefined;
        const skins = CurrentProfile.value.SkinInfo || [];
        return skins.find((i: any) => i.state === "ACTIVE");
    });

    const CurrentProfileCape = computed(() => {
        if (!CurrentProfile.value || CurrentProfile.value.Type !== "msa") return undefined;
        const capes = CurrentProfile.value.CapeInfo || [];
        return capes.find((i: any) => i.state === "ACTIVE")?.url;
    });

    // -------------------- Actions --------------------
    const openCreateModal = () => createModal.value?.showModal();
    const onRemoveProfile = () => removeModal.value?.show();

    async function createLegacyProfile() {
        if (!offlineValid.value) return;
        const uuid = offlineUuidMode.value === "custom" ? offlineUuid.value : undefined;
        await profileManager.createLegacy(offlineUsername.value, offlineUuidMode.value || "standard", uuid);
        cleanup();
    }

    async function switchNextProfile() {
        await profileManager.switchNext();
    }

    async function removeCurrentProfile() {
        await profileManager.removeCurrent();
        cleanup();
    }

    function onCreateNewProfile(startType: "msa" | "legacy" | "yggdrasil") {
        type.value = startType;
        step.value = 2;
        if (startType === "msa") profileManager.startMsaLogin(locale.value);
    }

    // 弹窗 cleanup
    async function cleanup() {
        step.value = 1;
        type.value = undefined;
        offlineUsername.value = "";
        offlineUuidMode.value = "";
        offlineUuid.value = "";
        createModal.value?.close();
        removeModal.value?.close();
        await profileManager.cleanup();
    }

    // -------------------- Mounted --------------------
    onMounted(async () => {
        await profileManager.load();
    });
</script>

<template>
    <div class="w-full h-[calc(100vh-128px-64px)] p-6">
        <!-- 顶部操作 -->
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

        <!-- 主体 -->
        <div class="flex-1 mt-4 h-full">
            <!-- 没有档案 -->
            <BankCard
                v-if="Profiles.length === 0"
                class="w-full! h-[calc(100%-62px)]! aspect-auto! mt-4"
                :style="{ '--bgc-perc': matchTheme('dark') ? '1.03%' : '0.75%' }">
                <div class="w-full h-full flex flex-col gap-4 justify-center items-center">
                    <i class="icon-[material-symbols--frame-exclamation-rounded] size-22"></i>
                    <span class="text-xl">{{ $t("Main.r/Profile.Profiles.NoProfile") }}</span>
                </div>
            </BankCard>

            <!-- 有档案 -->
            <div v-else class="grid grid-cols-[410px_1fr] grid-rows-[1fr] gap-4 h-full">
                <section class="h-full">
                    <div class="stack w-101.5 h-57.5">
                        <AccountCard
                            v-for="card in Profiles"
                            :key="card.Guid"
                            :profile="card"
                            :style="{ '--bgc-perc': matchTheme('dark') ? '4.23%' : '5.79%' }" />
                    </div>
                    <div class="card w-full bg-base-100 mt-2">
                        <div class="card-body">
                            <h2 class="card-title">{{ $t("Main.r/Profile.ActionButtons.__Title__") }}</h2>
                            <button class="btn btn-soft btn-primary mt-2 w-full" @click="switchNextProfile" :disabled="Profiles.length === 1">
                                <span class="text-sm">{{ $t("Main.r/Profile.ActionButtons.SwitchProfileNext") }}</span>
                            </button>
                            <button class="btn btn-soft btn-error mt-2 w-full" @click="onRemoveProfile" :disabled="Profiles.length === 1">
                                <span class="text-sm">
                                    {{
                                        CurrentProfile?.Type === "legacy"
                                            ? $t("Main.r/Profile.ActionButtons.Remove")
                                            : $t("Main.r/Profile.ActionButtons.Logout")
                                    }}
                                </span>
                            </button>
                        </div>
                    </div>
                </section>

                <!-- 玩家模型 -->
                <section class="h-full min-h-0">
                    <Player
                        v-if="CurrentProfile"
                        :skin-url="CurrentProfileSkin?.url"
                        :type="CurrentProfileSkin?.variant.toLowerCase() || 'slim'"
                        :cape-url="CurrentProfileCape" />
                </section>
            </div>
        </div>

        <!-- 创建档案弹窗 -->
        <dialog ref="createModal" class="modal">
            <div class="modal-box">
                <!-- Step 1 -->
                <section v-if="step === 1" class="flex flex-col items-center gap-2">
                    <h3 class="text-xl font-bold">{{ $t("Main.r/Profile.Modal.Add.Title") }}</h3>
                    <p class="text-sm opacity-50">{{ $t("Main.r/Profile.Modal.Add.Description") }}</p>
                    <div class="divider w-76 mx-auto my-0"></div>
                    <button class="btn bg-[#2F2F2F] text-white border-black w-66" @click="onCreateNewProfile('msa')">
                        <svg aria-label="Microsoft logo" width="24" height="24" viewBox="0 0 512 512">
                            <path d="M96 96H247V247H96" fill="#f24f23"></path>
                            <path d="M265 96V247H416V96" fill="#7eba03"></path>
                            <path d="M96 265H247V416H96" fill="#3ca4ef"></path>
                            <path d="M265 265H416V416H265" fill="#f9ba00"></path>
                        </svg>
                        <span class="text-sm">{{ $t("Main.r/Profile.Modal.Add.MsLogin") }}</span>
                    </button>
                    <button class="btn w-66" @click="onCreateNewProfile('legacy')" v-if="Profiles.some((i) => i.Type === 'msa')">
                        <i class="icon-[material-symbols--safety-check-off-outline-rounded] size-6 mr-1"></i>
                        <span class="text-sm">{{ $t("Main.r/Profile.Modal.Add.Offline") }}</span>
                    </button>
                    <div class="divider w-76 mx-auto my-0"></div>
                    <form method="dialog" class="w-66">
                        <button class="btn w-full" @click="cleanup">{{ $t("Main.r/Profile.Modal.Add.Cancel") }}</button>
                    </form>
                </section>

                <!-- Step 2 -->
                <section v-else-if="step === 2">
                    <!-- Legacy -->
                    <section v-if="type === 'legacy'" class="flex flex-col items-center gap-2">
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
                            <option value="standard">{{ $t("Main.r/Profile.Modal.Step2.Offline.Uuid.Select.Standrad") }}</option>
                            <option value="custom">{{ $t("Main.r/Profile.Modal.Step2.Offline.Uuid.Select.Custom") }}</option>
                        </select>
                        <input
                            v-if="offlineUuidMode === 'custom'"
                            type="text"
                            class="input outline-none validator"
                            :placeholder="$t('Main.r/Profile.Modal.Step2.Offline.Uuid.Input')"
                            pattern="[0-9a-fA-F\-]{36}"
                            v-model="offlineUuid" />
                        <div class="divider w-96 mx-auto my-0"></div>
                        <section class="w-76 grid grid-cols-2 gap-4">
                            <button class="btn btn-success w-full" @click="createLegacyProfile" :disabled="!offlineValid">
                                {{ $t("Main.r/Profile.Modal.Add.Create") }}
                            </button>
                            <button class="btn btn-soft w-full" @click="cleanup">
                                {{ $t("Main.r/Profile.Modal.Add.Back") }}
                            </button>
                        </section>
                    </section>

                    <!-- MSA -->
                    <section v-else-if="type === 'msa'" class="flex flex-col items-center gap-2">
                        <h3 class="text-xl font-bold">{{ $t("Main.r/Profile.Modal.Add.MsLogin") }}</h3>
                        <div class="divider w-96 mx-auto my-0"></div>
                        <span class="loading loading-dots loading-xl"></span>
                    </section>
                </section>
            </div>
        </dialog>

        <!-- 删除档案弹窗 -->
        <dialog ref="removeModal" class="modal">
            <div class="modal-box">
                <section class="flex flex-col items-center gap-2">
                    <h1 class="text-xl font-semibold">{{ $t("Main.r/Profile.Modal.Remove.Title") }}</h1>
                    <div class="divider w-96 mx-auto my-0"></div>
                    <p>{{ $t("Main.r/Profile.Modal.Remove.Content.Line1") }}</p>
                    <p class="mt-4">
                        {{
                            $t("Main.r/Profile.Modal.Remove.Content.Line2", {
                                name: CurrentProfile?.Name,
                                type: CurrentProfile?.Type ? $t(`Aside.AccountType.${CurrentProfile.Type}`) : "",
                            })
                        }}
                    </p>
                    <p>UUID {{ CurrentProfile?.Uuid }}</p>
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
