export interface IMrFilterItem {
    value: string;
    label?: string;
    translate?: string;
    ignorable?: boolean;
    checked: boolean;
    ignored: boolean;

    // 可能有 svg 图标
    icon?: string;
}
