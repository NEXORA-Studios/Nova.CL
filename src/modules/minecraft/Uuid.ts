import SparkMD5 from "spark-md5";

export function createOfflineUUID(name: string) {
    const hash = SparkMD5.hash("OfflinePlayer:" + name);

    return {
        dash: `${hash.slice(0, 8)}-${hash.slice(8, 12)}-3${hash.slice(13, 16)}-a${hash.slice(17, 20)}-${hash.slice(20)}`,
        raw: hash,
    };
}

export const uuid = {
    createOfflineUUID,
};
