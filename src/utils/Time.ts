export function expiresInToUnix(expiresInSeconds: number): number {
    return Math.floor(Date.now()) + expiresInSeconds;
}
