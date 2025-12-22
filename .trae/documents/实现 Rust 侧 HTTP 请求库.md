1. 在 Cargo.toml 中添加 reqwest 依赖，支持 JSON 功能和异步请求
2. 创建 http 目录结构，包含：

   * mod.rs - 导出 HTTP 库核心功能

   * client.rs - HTTP 客户端实现，支持 GET、POST、PUT、DELETE 等请求方法

   * types.rs - 请求/响应类型定义

   * command.rs - 实现 Tauri 命令，供 lib.rs 导入使用
3. 实现 HTTP 客户端的核心功能，包括请求构建、发送和响应处理
4. 在 command.rs 中创建 tauri::command 函数，调用 HTTP 客户端方法
5. 在 lib.rs 中导入并注册这些 Tauri 命令

