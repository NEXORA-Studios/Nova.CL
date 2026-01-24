# Internal IPC (Frontend to Backend)

## 前端侧实现细节

```typescript
/** 前端调用后端的请求 */
interface CallRequest<T = unknown> {
    /** 协议版本号 */
    version: IpcVersion;

    /** 唯一 ID，由前端生成 */
    id: number;

    /** 命令名 */
    topic: string;

    /** 请求数据 */
    payload?: T;

    /** 时间戳（Unix ms） */
    timestamp: number;
}

/** 所有前端发出的 IPC 消息 */
type IpcSendMessage<T = unknown> = CallRequest<T>;
```

## 后端侧实现细节

```rust
/// 前端请求后端的消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallRequest<T> {
    /// 协议版本号
    pub version: u8,
    /// 唯一 ID
    pub id: u64,
    /// 命令名
    pub topic: String,
    /// 请求数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
    /// 时间戳（Unix ms）
    pub timestamp: u64,
}
```
