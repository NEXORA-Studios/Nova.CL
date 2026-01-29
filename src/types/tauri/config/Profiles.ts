export type ProfileType = "msa" | "legacy" | "yggdrasil";

interface BaseProfile {
    Guid: string;
    Type: ProfileType;
    Uuid: string;
    Name: string;
    AccessToken: string;
    RefreshToken: string;
}

export interface MsaProfile extends BaseProfile {
    Type: "msa";
    MsaExpiresAt: number;
    McExpiresAt: number;
    SkinInfo: Array<any>;
    CapeInfo: Array<any>;
}

export interface LegacyProfile extends BaseProfile {
    Type: "legacy";
}

export interface YggdrasilProfile extends BaseProfile {
    Type: "yggdrasil";
    YggdrasilSite?: string;
    YggdrasilRegister?: string;
    YggdrasilSiteName?: string;
}

export type Profile = MsaProfile | LegacyProfile | YggdrasilProfile;

export interface ProfileConfig {
    Current: string;
    Profile: Profile[];
}
