## 实现多类型 TOML 配置文件系统（含 ChaCha20 加密支持）

### 1. 项目结构设计

* 创建 `src-tauri/src/config` 目录，用于存放配置相关代码

* 采用归一化设计，核心功能共享，不同配置类型分离

### 2. 依赖管理

* 在 `src-tauri/Cargo.toml` 中添加依赖：

  * `toml`：TOML 文件序列化/反序列化

  * `serde`：数据结构序列化支持

  * `thiserror`：友好的错误处理

  * `once_cell`：线程安全的全局配置访问

  * `tauri`：获取应用数据目录和安全存储

  * `chacha20poly1305`：提供 ChaCha20-Poly1305 加密算法

  * `base64`：用于编码加密后的数据

  * `rand`：生成随机数

### 3. 核心文件实现

#### 3.1 `config/mod.rs`

* 定义配置模块的入口

* 实现统一的配置加载/保存核心逻辑

* 导出三种配置类型的访问接口

#### 3.2 `config/types.rs`

* 定义三种配置的数据结构：

  * `GlobalConfig`：全局 Config.toml 配置

  * `ProfileConfig`：Profile.toml 账户配置

  * `InstanceConfig`：单个实例配置

* 所有配置结构均支持 serde 序列化/反序列化

* 为需要加密的字段添加特殊处理

#### 3.3 `config/error.rs`

* 定义配置相关的错误类型

* 支持不同配置类型的错误区分

#### 3.4 `config/manager.rs`

* 实现配置管理器，负责：

  * 配置文件路径管理

  * 统一的加载/保存逻辑

  * 配置缓存和访问控制

#### 3.5 `config/crypto.rs`

* 实现加密/解密功能：

  * 密钥管理

  * ChaCha20-Poly1305 字段加密

  * ChaCha20-Poly1305 字段解密

### 4. 配置类型设计

#### 4.1 GlobalConfig (Config.toml)

* 应用程序全局配置

* 示例配置项：

  * 应用主题

  * 默认语言

  * 自动更新设置

  * 日志级别

#### 4.2 ProfileConfig (Profile.toml)

* 账户信息配置

* 示例配置项：

  * `type`：账户类型（msa/legacy）

  * `refresh_token`：加密存储的刷新令牌

  * `uuid`：用户唯一标识符

  * 账户偏好设置

  * 历史记录

#### 4.3 InstanceConfig

* 单个实例配置

* 示例配置项：

  * 实例名称

  * 实例 URL

  * 实例特定设置

  * 连接状态

### 5. ChaCha20 加密功能实现

#### 5.1 加密策略

* 采用 ChaCha20-Poly1305 算法进行对称加密

* 加密密钥使用 Tauri 的安全存储管理

* 每次加密生成随机的 nonce（96位）

* 加密结果包含 nonce、认证标签和密文

* 加密数据使用 base64 编码，方便存储在 TOML 文件中

#### 5.2 密钥管理

* 主密钥通过 Tauri 的 `tauri::api::path::app_data_dir` 获取应用数据目录

* 使用应用特定的标识符生成主密钥

* 密钥在应用启动时加载，缓存在内存中

#### 5.3 字段加密

* 为需要加密的字段实现自定义的 serde 序列化/反序列化

* 在序列化时自动加密敏感字段

* 在反序列化时自动解密敏感字段

* 支持嵌套结构中的字段加密

#### 5.4 加密字段示例

```rust
#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub r#type: String,
    #[serde(with = "encrypted_field")]
    pub refresh_token: String,
    pub uuid: String,
    // 其他字段...
}
```

### 6. 功能实现

#### 6.1 统一的配置加载

* 支持从不同路径加载不同类型的配置

* 实现默认配置生成

* 自动创建配置文件（如果不存在）

* 加载时自动解密敏感字段

#### 6.2 配置访问

* 为每种配置类型提供独立的访问接口

* 线程安全的配置访问

* 支持配置项的读取和修改

#### 6.3 配置保存

* 保存时自动加密敏感字段

* 自动保存配置变更

* 支持手动触发保存

* 支持批量保存

### 7. 集成到项目中

#### 7.1 初始化

* 在 `lib.rs` 中初始化配置系统

* 加载全局配置和默认配置文件

* 初始化加密密钥

#### 7.2 Tauri 命令

* 提供 Tauri 命令，允许前端：

  * 读取全局配置

  * 更新全局配置

  * 管理账户配置（含加密字段）

  * 管理实例配置

### 8. 目录结构

```
src-tauri/src/config/
├── mod.rs          # 模块入口
├── error.rs        # 错误定义
├── manager.rs      # 配置管理器
├── types.rs        # 配置数据结构
├── crypto.rs       # ChaCha20 加密/解密功能
└── utils.rs        # 工具函数
```

### 9. 实现优势

#### 9.1 归一化设计

* 共享核心加载/保存逻辑

* 统一的错误处理机制

* 一致的 API 设计

#### 9.2 类型安全

* 每种配置类型都有明确的数据结构

* 编译时类型检查

* 自动生成序列化/反序列化代码

#### 9.3 加密安全

* 使用行业标准的 ChaCha20-Poly1305 加密算法

* 安全的密钥管理

* 自动的字段加密/解密

* 防止敏感信息泄露

#### 9.4 性能高效

* ChaCha20 在现代 CPU 上性能优异

* 不需要硬件加速支持

* 低功耗设备上表现更好

#### 9.5 扩展性

* 易于添加新的配置类型

* 支持配置项的版本管理

* 支持配置迁移

* 支持新增加密字段

#### 9.6 可靠性

* 良好的错误处理

* 配置文件的自动备份（可选）

* 支持配置验证

### 10. 示例配置文件

#### 10.1 Config.toml

```toml
[launch.basic]
version_indie_type="disabled" # disabled | modded | snapshot | modded_or_snapshot | all
launcher_visibility="immediately_quit" # immediately_quit | hide_then_quit | hide_then_show | minimize | constant
prefer_ip_stack=4 # 4=ipv4 | 0=default | 6=ipv6
selected_java="db495003-022d-42ae-a27f-a933997d63cb"

[[launcher.basic.java]]
id="db495003-022d-42ae-a27f-a933997d63cb"
path="..."
type="jdk" # jre | jdk
version=17

[launch.rams]
auto_ram=true
custom_ram=8192 # MB
pre_swap=false

[launch.advanced]
renderer="default" # default | llvmpipe | d3d12 | zink
jvm_args="-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Djdk.lang.Process.allowAmbiguousCommands=true -Dfml.ignoreInvalidMinecraftCertificates=True -Dfml.ignorePatchDiscrepancies=True -Dlog4j2.formatMsgNoLookups=true -DproxySet=false -Djava.net.useSystemProxies=false"
game_args=""
pre_command=""
disable_retrowrapper=false
use_discrete_gpu=true
use_java_exe=false

[customize]
theme="auto" # light | auto | dark
language="auto" # auto | zh-CN | en-US

[other.download]
download_source="offical" # offical | balance | mirror
version_source="offical" # offical | balance | mirror
max_concurrent=64 # 1 <= max_concurrent <= 256
max_bandwidth=-1 # -1 无限制，0.1 <= max_bandwidth <= 20
postselect_instance=true
update_authlib=true

[other.comp]
source="offical" # offical | balance | mirror
ignore_quilt=false
detect_clipboard=false

[other.accessibility]
release_note=false
snapshot_note=false
auto_chinese=true

[other.launcher]
update_method="auto" # auto | notice | major_notice | disable
channel="ender" # overworld | nether | ender
notification="all" # all | major | disable
cache_dir="" # empty=default, or a valid path

[other.network]
use_doh=false
use_system_proxy=false
use_custom_proxy=false
custom_proxy_uri=""
custom_proxy_account=""
custom_proxy_password=""

[other.debug]
debug_mode=false
```

#### 10.2 Profile.toml（含加密字段）

```toml
[[profile]]
type="msa"
refresh_token="base64_encoded_encrypted_refresh_token"
uuid="..."

[[profile]]
type="legacy"
id="LingyunAwA"
uuid="6843823712f3415fbd30b07a2f0b4efa"
```

#### 10.3 Instance Config

```toml
name="CABIN：Create Above & Beyond In Newer"
description=""
mc_version="1.20.1"
loader_type="forge"
loader_version="43.7.0"
launch_count=0
category=0
```

### 11. 测试

* 实现单元测试，测试每种配置类型的加载/保存

* 测试配置的并发访问

* 测试配置的错误处理

* 测试 ChaCha20 加密/解密功能

* 测试加密字段的序列化/反序列化

### 12. 文档

* 详细的代码注释

* 配置项的使用说明

* 加密机制的设计文档

* 配置系统的架构文档

这个实现计划将为 Nova.CL 应用提供一个统一、灵活、可靠的多类型 TOML 配置文件系统，支持全局配置、账户配置和实例配置的管理，并使用 ChaCha20-Poly1305 算法为敏感字段提供安全高效的加密存储。
