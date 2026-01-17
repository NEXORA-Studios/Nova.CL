declare global {
    interface Console {
        tLog({ category: string, message: string }): void;
        tTrace({ category: string, message: string }): void;
        tDebug({ category: string, message: string }): void;
        tInfo({ category: string, message: string }): void;
        tWarn({ category: string, message: string }): void;
        tError({ category: string, message: string }): void;
    }
}

export {};
