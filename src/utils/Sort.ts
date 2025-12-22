/**
 * 根据指定顺序数组排序目标数组
 * @param arr 要排序的数组
 * @param order 顺序数组，决定排序优先级
 * @returns 排序后的新数组
 */
export function sortByOrder<T>(arr: T[], order: T[]): T[] {
    const orderMap = new Map(order.map((item, index) => [item, index]));
    return [...arr].sort((a, b) => {
        const indexA = orderMap.get(a) ?? Infinity;
        const indexB = orderMap.get(b) ?? Infinity;
        return indexA - indexB;
    });
}

export function sortByOrderField<T>(
    arr: T[],
    order: Array<string | number>, // 顺序数组
    field: keyof T // 用于排序的字段
): T[] {
    const orderMap = new Map(order.map((item, index) => [item, index]));
    return [...arr].sort((a, b) => {
        const indexA = orderMap.get(a[field] as string | number) ?? Infinity;
        const indexB = orderMap.get(b[field] as string | number) ?? Infinity;
        return indexA - indexB;
    });
}
