export class RuntimeCache {
    private static cache: Record<string, any> = {}

    public static Set(key: string, value: any) {
        this.cache[key] = value
    }

    public static Get(key: string) {
        return this.cache[key]
    }

    public static GetWithDefault(key: string, defaultValue: any) {
        return this.Get(key) ?? defaultValue
    }

    public static Exists(key: string) {
        return this.cache[key] !== undefined
    }

    public static Remove(key: string) {
        delete this.cache[key]
    }

    public static Clear() {
        this.cache = {}
    }
}
