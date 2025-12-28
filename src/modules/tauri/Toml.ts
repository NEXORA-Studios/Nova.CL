import { invoke } from "@tauri-apps/api/core";
import { ITauriTypes } from "@/types";

// 配置管理类
export class TomlClient {
    /**
     * 获取全局配置
     */
    static async getGlobalConfig(): Promise<ITauriTypes.TOML.GlobalConfig> {
        try {
            const config = await invoke<ITauriTypes.TOML.GlobalConfig>("get_global_config_cmd");
            return config;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 保存全局配置
     */
    static async saveGlobalConfig(config: ITauriTypes.TOML.GlobalConfig): Promise<void> {
        try {
            await invoke<void>("save_global_config_cmd", { config });
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 获取账户配置
     */
    static async getProfileConfig(): Promise<ITauriTypes.TOML.ProfileConfig> {
        try {
            const config = await invoke<ITauriTypes.TOML.ProfileConfig>("get_profile_config_cmd");
            return config;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 保存账户配置
     */
    static async saveProfileConfig(config: ITauriTypes.TOML.ProfileConfig): Promise<void> {
        try {
            await invoke<void>("save_profile_config_cmd", { config });
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 获取集合配置
     */
    static async getCollectionConfig(): Promise<ITauriTypes.TOML.CollectionConfig> {
        try {
            const config = await invoke<ITauriTypes.TOML.CollectionConfig>("get_collection_config_cmd");
            return config;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 保存集合配置
     */
    static async saveCollectionConfig(config: ITauriTypes.TOML.CollectionConfig): Promise<void> {
        try {
            await invoke<void>("save_collection_config_cmd", { config });
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 获取指定实例配置
     */
    static async getInstanceConfig(instancePath: string): Promise<ITauriTypes.TOML.InstanceConfig> {
        try {
            const config = await invoke<ITauriTypes.TOML.InstanceConfig>("get_instance_config_cmd", { instancePath });
            return config;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 保存实例配置
     * @param instancePath 实例路径
     * @param config 实例配置
     */
    static async saveInstanceConfig(instancePath: string, config: ITauriTypes.TOML.InstanceConfig): Promise<void> {
        try {
            await invoke<void>("save_instance_config_cmd", { instancePath, config });
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 删除实例配置
     */
    static async deleteInstanceConfig(instancePath: string): Promise<void> {
        try {
            await invoke<void>("delete_instance_config_cmd", { instancePath });
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 列出所有实例配置ID
     */
    static async listInstanceConfigs(): Promise<string[]> {
        try {
            const instanceIds = await invoke<string[]>("list_instance_configs_cmd");
            return instanceIds;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 解密字符串
     */
    static async decryptString(encrypted: string): Promise<string> {
        try {
            const decrypted = await invoke<string>("decrypt_string_cmd", { encrypted });
            return decrypted;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }

    /**
     * 加密字符串
     */
    static async encryptString(plaintext: string): Promise<string> {
        try {
            const encrypted = await invoke<string>("encrypt_string_cmd", { plaintext });
            return encrypted;
        } catch (error) {
            throw error as ITauriTypes.TOML.ConfigError;
        }
    }
}

// 导出便捷的配置管理对象
export const toml = {
    getGlobalConfig: TomlClient.getGlobalConfig,
    saveGlobalConfig: TomlClient.saveGlobalConfig,
    getProfileConfig: TomlClient.getProfileConfig,
    saveProfileConfig: TomlClient.saveProfileConfig,
    getCollectionConfig: TomlClient.getCollectionConfig,
    saveCollectionConfig: TomlClient.saveCollectionConfig,
    getInstanceConfig: TomlClient.getInstanceConfig,
    saveInstanceConfig: TomlClient.saveInstanceConfig,
    deleteInstanceConfig: TomlClient.deleteInstanceConfig,
    listInstanceConfigs: TomlClient.listInstanceConfigs,
    decryptString: TomlClient.decryptString,
    encryptString: TomlClient.encryptString,
};
