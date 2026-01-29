import { ref, computed } from "vue";
import { v7 as uuidv7 } from "uuid";
import { ITauriTypes } from "@/types";
import { TauriConfig, TauriHttpServer, McUuid, McMsa } from "@/modules";
import { expiresInToUnix } from "@/utils";
import { openUrl } from "@tauri-apps/plugin-opener";

type Profile = ITauriTypes.Config.ProfileConfig["Profile"][number];

function normalizeProfiles(v: unknown): Profile[] {
    if (!Array.isArray(v)) return [];
    return v.filter(Boolean) as Profile[];
}

export function useProfileManager() {
    /* -------------------- state -------------------- */

    const profiles = ref<Profile[]>([]);
    const currentGuid = ref<string | undefined>();

    const currentProfile = computed(() => profiles.value.find((p) => p.Guid === currentGuid.value));

    /* -------------------- load -------------------- */

    async function load() {
        const [listStr, current] = await Promise.all([
            TauriConfig.get<string>("Profiles.Profile"),
            TauriConfig.get<string>("Profiles.Current"),
        ]);

        const list: Profile[] = JSON.parse(listStr);
        profiles.value = normalizeProfiles(list);
        currentGuid.value = current;
    }

    /* -------------------- switch -------------------- */

    async function switchNext() {
        const list = profiles.value;
        if (list.length <= 1) return;

        const idx = list.findIndex((p) => p.Guid === currentGuid.value);
        const next = list[(idx + 1) % list.length];
        if (!next) return;

        currentGuid.value = next.Guid;

        await TauriConfig.set("Profiles.Current", next.Guid);
    }

    /* -------------------- remove -------------------- */

    async function removeCurrent() {
        const guid = currentGuid.value;
        if (!guid) return;

        const list = [...profiles.value];
        const idx = list.findIndex((p) => p.Guid === guid);
        if (idx === -1) return;

        list.splice(idx, 1);

        currentGuid.value = list.length > 0 ? list[Math.min(idx, list.length - 1)].Guid : undefined;
        profiles.value = list;

        await Promise.all([
            TauriConfig.set("Profiles.Profile", JSON.stringify(list)),
            TauriConfig.set("Profiles.Current", currentGuid.value ?? ""),
        ]);
    }

    /* -------------------- legacy -------------------- */

    async function createLegacy(username: string, mode: "standard" | "custom", uuid?: string) {
        const id = mode === "custom" && uuid ? uuid : McUuid.createOfflineUUID(username).dash;

        const profile: Profile = {
            Guid: uuidv7(),
            Type: "legacy",
            Name: username,
            Uuid: id,
            AccessToken: "",
            RefreshToken: "",
        };

        const list = [...profiles.value, profile];

        profiles.value = list;
        currentGuid.value = profile.Guid;

        await Promise.all([TauriConfig.set("Profiles.Profile", JSON.stringify(list)), TauriConfig.set("Profiles.Current", profile.Guid)]);
    }

    /* -------------------- MSA -------------------- */

    async function startMsaLogin(locale: string, onStep?: (step: number) => void, onError?: (e: any) => void, onSuccess?: () => void) {
        const env = import.meta.env;

        const LOGIN_URI =
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?" +
            new URLSearchParams({
                client_id: env.OAUTH_MS_CLIENT_ID,
                response_type: "code",
                redirect_uri: env.OAUTH_REDIRECT_URI_BASE + env.OAUTH_MS_REDIRECT_URI,
                response_mode: "query",
                scope: "XboxLive.signin offline_access",
            }).toString();

        try {
            await TauriHttpServer.start(36993, locale);

            await TauriHttpServer.listenOAuthCode(async (payload) => {
                if (payload.path !== "/oauth/callback" || !payload.query) return;

                const query = new URLSearchParams(payload.query);
                const code = query.get("code");
                if (!code) return;

                const result = await McMsa.loginFromCode(code, (s) => onStep?.(s));

                let profile = profiles.value.find((p) => p.Type === "msa" && p.Uuid === result.uuid);

                if (!profile) {
                    profile = {
                        Guid: uuidv7(),
                        Type: "msa",
                        Name: result.name ?? "Unknown",
                        Uuid: result.uuid ?? "Unknown",
                        AccessToken: result.msaAccessToken ?? "",
                        RefreshToken: result.msaRefreshToken ?? "",
                        MsaExpiresAt: expiresInToUnix(result.msaExpiresIn * 1000),
                        McExpiresAt: expiresInToUnix(result.mcExpiresIn * 1000),
                        SkinInfo: result.skins ?? [],
                        CapeInfo: result.capes ?? [],
                    };

                    profiles.value.push(profile);
                }

                currentGuid.value = profile.Guid;

                console.tTrace({
                    category: "ProfileManager",
                    message: `更新 currentGuid: ${currentGuid.value}`,
                });

                await Promise.all([
                    TauriConfig.set("Profiles.Profile", JSON.stringify(profiles.value)),
                    TauriConfig.set("Profiles.Current", profile.Guid),
                ]);

                await cleanup();
                onSuccess?.();
            });

            await openUrl(LOGIN_URI);
        } catch (e) {
            onError?.(e);
        }
    }

    /* -------------------- cleanup -------------------- */

    async function cleanup() {
        try {
            TauriHttpServer.unlistenOAuthCode();
            await TauriHttpServer.stop();
        } catch {
            console.tWarn({
                category: "ProfileManager",
                message: "Graceful cleanup the session failed, but does not raise error.",
            });
        }
    }

    return {
        // state
        profiles,
        currentGuid,
        currentProfile,

        // actions
        load,
        switchNext,
        removeCurrent,
        createLegacy,
        startMsaLogin,
        cleanup,
    };
}
