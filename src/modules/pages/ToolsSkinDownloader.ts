import { TauriHTTPClient } from "..";

// interface MojangSessionProfile {
//     id: string;
//     name: string;
//     properties: { name: string; value: string }[];
//     profileActions: any[];
// }

// interface MojangBase64Profile {
//     timestamp: number;
//     profileId: string;
//     profileName: string;
//     textures: {
//         SKIN: { url: string };
//         CAPE: { url: string };
//     };
// }

export class ToolsSkinDownloader {
    private selectSource: string = "mojang";
    private selectType: string = "skin";
    private inputUsername: string = "";

    // private _MirrorUri = "https://cdn.akaere.online/";

    public setMeta(selectSource: string, selectType: string, inputUsername: string) {
        this.selectSource = selectSource;
        this.selectType = selectType;
        this.inputUsername = inputUsername;
    }

    private async getUuid() {
        // const requestUri = `https://uapis.cn/api/v1/game/minecraft/userinfo?username=${this.inputUsername}`;
        // const response = await axios.get<{ code: number; uuid: string; name: string; skin_url: string }>(requestUri);
        // return { uuid: response.data.uuid, skin_url: response.data.skin_url };

        const response = (
            await TauriHTTPClient.get<{ uuid: string; skin_url: string }>(
                `https://uapis.cn/api/v1/game/minecraft/userinfo?username=${this.inputUsername}`
            )
        ).body;
        return { uuid: response.uuid, skin_url: response.skin_url };
    }

    public async getDownloadUri() {
        const { uuid, skin_url } = await this.getUuid();

        switch (this.selectSource) {
            case "mojang":
                // const requestUri = `${this._MirrorUri}https://sessionserver.mojang.com/session/minecraft/profile/${uuid}`;
                // const response = await axios.get<MojangSessionProfile>(requestUri);
                // const profile: MojangBase64Profile = JSON.parse(atob(response.data.properties[0]!.value!));
                switch (this.selectType) {
                    case "skin":
                        return skin_url;
                    // return profile.textures.SKIN.url;
                    case "cape":
                    // Fall-through
                    // return profile.textures.CAPE.url;
                    default:
                        throw new Error("Current Type for Current Source is Not Supported");
                }
            // case "crafatar":
            //     switch (this.selectType) {
            //         case "avatar":
            //             return `https://crafatar.com/avatars/${uuid}`;
            //         case "head":
            //             return `https://crafatar.com/renders/head/${uuid}`;
            //         case "body":
            //             return `https://crafatar.com/renders/body/${uuid}`;
            //         case "skin":
            //             return `https://crafatar.com/skins/${uuid}`;
            //         case "cape":
            //             return `https://crafatar.com/capes/${uuid}`;
            //         default:
            //             throw new Error("Current Type for Current Source is Not Supported");
            //     }
            case "minotar":
                switch (this.selectType) {
                    case "avatar":
                        return `https://minotar.net/avatar/${uuid}/128`;
                    case "helm":
                        return `https://minotar.net/helm/${uuid}/128`;
                    case "isometric":
                        return `https://minotar.net/cube/${uuid}/128`;
                    case "body":
                        return `https://minotar.net/body/${uuid}/128`;
                    case "bust":
                        return `https://minotar.net/bust/${uuid}/128`;
                    case "skin":
                        return `https://minotar.net/skin/${uuid}`;
                    default:
                        throw new Error("Current Type for Current Source is Not Supported");
                }
            default:
                throw new Error("Current Download Source is Not Supported");
        }
    }
}
