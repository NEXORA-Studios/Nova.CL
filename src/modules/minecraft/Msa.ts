import { IMinecraftTypes } from "@/types";
import { MsaLoginResult } from "@/types/minecraft/Msa";
import { TauriHTTPClient } from "..";

export class MsaLoginError extends Error {
    code: number;
    step: number;

    constructor(message: string, code = -1, step = -1) {
        super(message);
        this.name = "MsaLoginError";
        this.code = code;
        this.step = step;
    }
}

export async function login(
    key: { code?: string; refresh_token?: string },
    grant_type: string,
    updateStepFn: (step: number) => void
): Promise<MsaLoginResult> {
    const $env = import.meta.env;
    const payload = key.code ? { code: key.code } : key.refresh_token ? { refresh_token: key.refresh_token } : {};

    // Step 1: MSA Token
    updateStepFn(1);
    let msaResp: IMinecraftTypes.Msa.MsaTokenResponse;

    try {
        msaResp = (
            await TauriHTTPClient.post<IMinecraftTypes.Msa.MsaTokenResponse>(
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
                { "Content-Type": "application/x-www-form-urlencoded" },
                {
                    client_id: $env.OAUTH_MS_CLIENT_ID,
                    client_secret: $env.OAUTH_MS_CLIENT_SECRET,
                    redirect_uri: $env.OAUTH_REDIRECT_URI_BASE + $env.OAUTH_MS_REDIRECT_URI,
                    ...payload,
                    grant_type,
                    scope: "XboxLive.signin offline_access",
                }
            )
        ).body;
    } catch (e: any) {
        // 只做一件事：语义转换
        throw new MsaLoginError(e?.message || "Failed to request MSA token", e?.code ?? -1, 1);
    }

    if ("error" in msaResp) {
        throw new MsaLoginError(msaResp.error_description || "MSA token error", msaResp.error_codes?.[0] ?? -1, 1);
    }

    const ACCESS_TOKEN = (msaResp as IMinecraftTypes.Msa.MsaTokenSuccess).access_token;
    const REFRESH_TOKEN = (msaResp as IMinecraftTypes.Msa.MsaTokenSuccess).refresh_token;
    const MSA_EXPIRES_IN = (msaResp as IMinecraftTypes.Msa.MsaTokenSuccess).expires_in;

    // Step 2: XBL Token
    updateStepFn(2);
    let xblResp: IMinecraftTypes.Msa.XblTokenSuccess;
    try {
        xblResp = (
            await TauriHTTPClient.post<IMinecraftTypes.Msa.XblTokenSuccess>(
                "https://user.auth.xboxlive.com/user/authenticate",
                { "Content-Type": "application/json" },
                {
                    Properties: {
                        AuthMethod: "RPS",
                        SiteName: "user.auth.xboxlive.com",
                        RpsTicket: `d=${ACCESS_TOKEN}`,
                    },
                    RelyingParty: "http://auth.xboxlive.com",
                    TokenType: "JWT",
                }
            )
        ).body;
    } catch {
        try {
            // fallback（微软历史遗留的迷惑行为）
            xblResp = (
                await TauriHTTPClient.post<IMinecraftTypes.Msa.XblTokenSuccess>(
                    "https://user.auth.xboxlive.com/user/authenticate",
                    { "Content-Type": "application/json" },
                    {
                        Properties: {
                            AuthMethod: "RPS",
                            SiteName: "user.auth.xboxlive.com",
                            RpsTicket: ACCESS_TOKEN,
                        },
                        RelyingParty: "http://auth.xboxlive.com",
                        TokenType: "JWT",
                    }
                )
            ).body;
        } catch (e: any) {
            throw new MsaLoginError(e?.message || "Failed to request XBL token", e?.code ?? -1, 2);
        }
    }

    const XBL_TOKEN = (xblResp as IMinecraftTypes.Msa.XblTokenSuccess).Token;
    const UHS = (xblResp as IMinecraftTypes.Msa.XblTokenSuccess).DisplayClaims.xui[0].uhs;

    // Step 3: XSTS
    updateStepFn(3);
    let xstsResp: IMinecraftTypes.Msa.XstsTokenSuccess;

    try {
        xstsResp = (
            await TauriHTTPClient.post<IMinecraftTypes.Msa.XstsTokenSuccess>(
                "https://xsts.auth.xboxlive.com/xsts/authorize",
                { "Content-Type": "application/json" },
                {
                    Properties: {
                        SandboxId: "RETAIL",
                        UserTokens: [XBL_TOKEN],
                    },
                    RelyingParty: "rp://api.minecraftservices.com/",
                    TokenType: "JWT",
                }
            )
        ).body;
    } catch (e: any) {
        throw new MsaLoginError(e?.message || "Failed to request XSTS token", e?.code ?? -1, 3);
    }

    if ((xstsResp as IMinecraftTypes.Msa.XstsTokenSuccess).DisplayClaims.xui[0].uhs !== UHS) {
        throw new MsaLoginError("UHS mismatch between XBL and XSTS");
    }

    // Step 4: Minecraft Token
    updateStepFn(4);
    let mcTokenResp: IMinecraftTypes.Msa.MinecraftTokenResponse;

    try {
        mcTokenResp = (
            await TauriHTTPClient.post<IMinecraftTypes.Msa.MinecraftTokenResponse>(
                "https://api.minecraftservices.com/authentication/login_with_xbox",
                { "Content-Type": "application/json" },
                {
                    identityToken: `XBL3.0 x=${UHS};${(xstsResp as IMinecraftTypes.Msa.XstsTokenSuccess).Token}`,
                }
            )
        ).body;
    } catch (e: any) {
        throw new MsaLoginError(e?.message || "Failed to request Minecraft token", e?.code ?? -1, 4);
    }

    if ("errorMessage" in mcTokenResp) {
        throw new MsaLoginError(mcTokenResp.errorMessage);
    }

    const MC_ACCESS_TOKEN = mcTokenResp.access_token;
    const MC_EXPIRES_IN = mcTokenResp.expires_in;

    // Step 5: License Check
    updateStepFn(5);
    let storeResp: IMinecraftTypes.Msa.MCStoreResponse;
    try {
        storeResp = (
            await TauriHTTPClient.get<IMinecraftTypes.Msa.MCStoreResponse>("https://api.minecraftservices.com/entitlements/mcstore", {
                "Content-Type": "application/json",
                Authorization: `Bearer ${MC_ACCESS_TOKEN}`,
            })
        ).body;
    } catch (e: any) {
        throw new MsaLoginError(e?.message || "Failed to request MCStore", e?.code ?? -1, 5);
    }

    if ("errorType" in storeResp) {
        throw new MsaLoginError(`Mojang API Error: ${storeResp.errorType}`, -1, 5);
    }

    const items = (storeResp as IMinecraftTypes.Msa.MCStoreSuccess).items ?? [];
    const hasJava = items.some((i) => i.name === "product_minecraft" || i.name === "game_minecraft");

    if (!hasJava) {
        throw new MsaLoginError("Minecraft Java Edition not purchased", -1, 5);
    }

    // Step 6: Profile
    updateStepFn(6);
    let profileResp: IMinecraftTypes.Msa.MinecraftProfileResponse;
    try {
        profileResp = (
            await TauriHTTPClient.get<IMinecraftTypes.Msa.MinecraftProfileResponse>("https://api.minecraftservices.com/minecraft/profile", {
                "Content-Type": "application/json",
                Authorization: `Bearer ${MC_ACCESS_TOKEN}`,
            })
        ).body;
    } catch (e: any) {
        throw new MsaLoginError(e?.message || "Failed to request Minecraft profile", e?.code ?? -1, 6);
    }

    if ("errorType" in profileResp) {
        throw new MsaLoginError(`Mojang API Error: ${profileResp.errorType}`, -1, 6);
    }

    const uuid = [
        (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).id.slice(0, 8),
        (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).id.slice(8, 12),
        (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).id.slice(12, 16),
        (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).id.slice(16, 20),
        (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).id.slice(20),
    ].join("-");

    return {
        msaExpiresIn: MSA_EXPIRES_IN,
        mcExpiresIn: MC_EXPIRES_IN,
        msaAccessToken: ACCESS_TOKEN,
        msaRefreshToken: REFRESH_TOKEN,
        mcAccessToken: MC_ACCESS_TOKEN,
        uuid,
        name: (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).name,
        skins: (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).skins,
        capes: (profileResp as IMinecraftTypes.Msa.MinecraftProfileSuccess).capes,
    };
}

export async function loginFromCode(code: string, updateStepFn: (step: number) => void): Promise<MsaLoginResult> {
    return login({ code }, "authorization_code", updateStepFn);
}

export async function loginFromRefreshToken(refresh_token: string, updateStepFn: (step: number) => void): Promise<MsaLoginResult> {
    return login({ refresh_token }, "refresh_token", updateStepFn);
}
