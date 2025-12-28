export interface MojangAPIError {
    errorMessage: string;
    path: string;
}

export interface MsaTokenSuccess {
    token_type: string;
    scope: string;
    expires_in: number;
    ext_expires_in: number;
    access_token: string;
    refresh_token: string;
}

export interface MsaTokenError {
    correlation_id: string;
    error: string;
    error_codes: number[];
    error_description: string;
    error_uri: string;
    timestamp: string;
    trace_id: string;
}

export interface MsaTokenResponse extends MsaTokenSuccess, MsaTokenError {}

export interface XblTokenSuccess {
    IssueInstant: string;
    NotAfter: string;
    Token: string;
    DisplayClaims: {
        xui: [
            {
                uhs: string;
            },
        ];
    };
}

export interface XstsTokenSuccess {
    IssueInstant: string;
    NotAfter: string;
    Token: string;
    DisplayClaims: {
        xui: [
            {
                uhs: string;
            },
        ];
    };
}

export interface MinecraftTokenSuccess {
    username: string;
    roles: any[];
    access_token: string;
    token_type: "Bearer";
    expires_in: number;
}

export type MinecraftTokenResponse = MinecraftTokenSuccess | MojangAPIError;

export interface MCStoreSuccess {
    items: { name: string; signature: string }[];
    signatures: string;
    keyId: 1;
}

export type MCStoreResponse = MCStoreSuccess | MojangAPIError;

export interface MinecraftProfileSuccess {
    id: string;
    name: string;
    skins: {
        id: string;
        state: string;
        url: string;
        variant: string;
        alias: string;
    }[];
    capes: {
        id: string;
        state: string;
        url: string;
    }[];
}

export interface MinecraftProfileError {
    path: string;
    errorType: string;
    error: number[];
    errorMessage: string;
    developerMessage: string;
}

export type MinecraftProfileResponse = MinecraftProfileSuccess | MinecraftProfileError;

export interface MsaLoginResult {
    msaExpiresIn: number
    mcExpiresIn: number;
    msaAccessToken: string;
    msaRefreshToken: string;

    mcAccessToken: string;

    uuid: string;
    name: string;
    skins: any[];
    capes: any[];
}
