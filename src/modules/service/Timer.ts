import { type HookFunction } from "@/types";

export class Timer {
    private static instance: Timer | null = null;
    private shortIntervalDuration = 500;
    private shortIntervalHook: HookFunction[] = [];
    private mediumIntervalDuration = 2500;
    private mediumIntervalHook: HookFunction[] = [];
    private longIntervalDuration = 5000;
    private longIntervalHook: HookFunction[] = [];

    private async performHookTask(hooks: HookFunction[], intervalName: string) {
        for (const hook of hooks) {
            try {
                await hook();
            } catch (error) {
                console.error(`Error in ${intervalName}:`, error);
            }
        }
    }

    private constructor(overrideIntervalDuration?: [number, number, number]) {
        if (overrideIntervalDuration && overrideIntervalDuration.length === 3) {
            [this.shortIntervalDuration, this.mediumIntervalDuration, this.longIntervalDuration] =
                overrideIntervalDuration;
        }

        setInterval(async () => {
            await this.performHookTask(this.shortIntervalHook, "short interval");
        }, this.shortIntervalDuration);

        setInterval(async () => {
            await this.performHookTask(this.mediumIntervalHook, "medium interval");
        }, this.mediumIntervalDuration);

        setInterval(async () => {
            await this.performHookTask(this.longIntervalHook, "long interval");
        }, this.longIntervalDuration);
    }

    static getInstance(): Timer {
        if (!Timer.instance) {
            Timer.instance = new Timer();
        }
        return Timer.instance;
    }

    public addShortIntervalHook(hook: HookFunction) {
        this.shortIntervalHook.push(hook);
    }
    public addMediumIntervalHook(hook: HookFunction) {
        this.mediumIntervalHook.push(hook);
    }
    public addLongIntervalHook(hook: HookFunction) {
        this.longIntervalHook.push(hook);
    }

    public removeShortIntervalHook(hook: HookFunction) {
        this.shortIntervalHook = this.shortIntervalHook.filter((h) => h !== hook);
    }
    public removeMediumIntervalHook(hook: HookFunction) {
        this.mediumIntervalHook = this.mediumIntervalHook.filter((h) => h !== hook);
    }
    public removeLongIntervalHook(hook: HookFunction) {
        this.longIntervalHook = this.longIntervalHook.filter((h) => h !== hook);
    }
}
