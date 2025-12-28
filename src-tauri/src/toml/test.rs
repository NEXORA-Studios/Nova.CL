use crate::toml::*;

/// 测试配置系统功能
#[allow(dead_code)]
pub fn test_config_system() {
    println!("Testing config system...");

    // 测试 1: 初始化配置系统
    println!("1. Testing config initialization...");
    match init_config() {
        Ok(_) => println!("   ✓ Config initialized successfully"),
        Err(e) => {
            println!("   ✗ Failed to initialize config: {}", e);
            return;
        }
    }

    // 测试 2: 测试全局配置
    println!("2. Testing global config...");

    // 获取全局配置
    match get_global_config() {
        Ok(config) => {
            println!("   ✓ Global config loaded successfully");
            println!("   - Theme: {}", config.customize.theme);
            println!("   - Language: {}", config.customize.language);
        }
        Err(e) => {
            println!("   ✗ Failed to load global config: {}", e);
        }
    }

    // 测试 3: 测试账户配置
    println!("3. Testing profile config...");

    // 获取账户配置
    match get_profile_config() {
        Ok(config) => {
            println!("   ✓ Profile config loaded successfully");
            println!("   - Number of profiles: {}", config.profile.len());
        }
        Err(e) => {
            println!("   ✗ Failed to load profile config: {}", e);
        }
    }

    // 测试 4: 测试实例配置
    println!("4. Testing instance config...");

    // 创建临时实例目录作为测试路径
    let temp_dir = std::env::temp_dir().join("NovaCLTestInstance");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let instance_path = temp_dir.to_str().unwrap();
    println!("   - Using test instance path: {}", instance_path);

    // 创建实例配置
    let instance_config = InstanceConfig {
        name: "Test Instance".to_string(),
        description: "A test instance".to_string(),
        mc_version: "1.20.1".to_string(),
        loader_type: "vanilla".to_string(),
        loader_version: "".to_string(),
        launch_count: 0,
        category: 0,
        icon: None,
        last_launch: None,
    };

    // 保存实例配置
    match save_instance_config(instance_path, &instance_config) {
        Ok(_) => println!("   ✓ Instance config saved successfully"),
        Err(e) => {
            println!("   ✗ Failed to save instance config: {}", e);
        }
    }

    // 测试读取实例配置
    match get_instance_config(instance_path) {
        Ok(loaded_config) => {
            println!("   ✓ Instance config loaded successfully");
            println!("   - Loaded instance name: {}", loaded_config.name);
        }
        Err(e) => {
            println!("   ✗ Failed to load instance config: {}", e);
        }
    }

    // 测试 5: 测试加密功能
    println!("5. Testing encryption...");

    // 创建测试账户
    let test_profile = Profile {
        guid: "bf24da30-1f3d-4c3f-84e6-0f08d58c8375".to_string(),
        r#type: "msa".to_string(),
        uuid: "aa2e9f5f-5349-4bab-b0ac-d9b658845263".to_string(),
        name: "Test User".to_string(),
        access_token: "".to_string(),
        refresh_token: "".to_string(),
        yggdrasil_site: None,
        yggdrasil_register: None,
        yggdrasil_site_name: None,
        picked: true,
        msa_expires_at: Some(0),
        mc_expires_at: Some(0),
        skin_info: Some("".to_string()),
        cape_info: Some("".to_string()),
    };

    let mut profile_config = ProfileConfig::default();
    profile_config.profile.push(test_profile);

    // 保存账户配置（自动加密）
    match save_profile_config(&profile_config) {
        Ok(_) => println!("   ✓ Profile with encrypted fields saved successfully"),
        Err(e) => {
            println!("   ✗ Failed to save profile with encrypted fields: {}", e);
        }
    }

    // 重新加载账户配置（自动解密）
    match get_profile_config() {
        Ok(config) => {
            println!("   ✓ Profile with encrypted fields loaded successfully");
            if let Some(profile) = config.profile.first() {
                println!("   - Profile type: {}", profile.r#type);
                println!("   - Profile UUID: {}", profile.uuid);
                // 注意：refresh_token 已被解密，但不应在日志中显示
                println!("   - Refresh token: [hidden]");
            }
        }
        Err(e) => {
            println!("   ✗ Failed to load profile with encrypted fields: {}", e);
        }
    }

    // 测试 6: 清理测试实例
    println!("6. Testing instance deletion...");

    match delete_instance_config(instance_path) {
        Ok(_) => println!("   ✓ Test instance config deleted successfully"),
        Err(e) => {
            println!("   ✗ Failed to delete test instance config: {}", e);
        }
    }

    // 删除临时目录
    std::fs::remove_dir_all(&temp_dir).unwrap();
    println!("   ✓ Test instance directory deleted successfully");

    println!("\nConfig system tests completed!");
}

// 仅在测试模式下编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        test_config_system();
    }
}
