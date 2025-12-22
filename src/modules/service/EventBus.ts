// 具体事件映射
interface AppEventMap {}

// 事件处理器类型
type EventHandler<T = unknown> = (payload: T) => void;

class EventBusFactory<TEvents> {
    private events: Map<keyof TEvents, EventHandler<unknown>[]> = new Map();

    on<K extends keyof TEvents>(event: K, handler: EventHandler<TEvents[K]>): void {
        if (!this.events.has(event)) {
            this.events.set(event, []);
        }
        this.events.get(event)!.push(handler as EventHandler<unknown>);
    }

    off<K extends keyof TEvents>(event: K, handler?: EventHandler<TEvents[K]>): void {
        if (!this.events.has(event)) return;

        if (handler) {
            const handlers = this.events.get(event)!;
            this.events.set(
                event,
                handlers.filter((h) => h !== handler)
            );
        } else {
            this.events.delete(event);
        }
    }

    emit<K extends keyof TEvents>(event: K, payload: TEvents[K]): void {
        const handlers = this.events.get(event);
        if (handlers) {
            handlers.forEach((handler) => handler(payload));
        }
    }

    // 获取所有事件名类型
    getEventNames(): (keyof TEvents)[] {
        return Array.from(this.events.keys());
    }
}

// 实例化全局 EventBus
export const EventBus = new EventBusFactory<AppEventMap>();
