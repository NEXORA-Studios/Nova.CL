import { ref, watch, computed, type Ref } from "vue";
import { modrinthApiAdapter } from "@/modules";
import type { IMrApi, IMrFilterItem } from "@/types";
import { unifyString } from "@/utils";

export class OffsetFilterModule {
    private _offset = ref(0);
    private _maximum = ref(0);

    get offset() {
        return this._offset.value;
    }

    get maximum() {
        return this._maximum.value;
    }

    set offset(value: number) {
        if (value < 0) value = 0;
        if (value > this.maximum) value = this.maximum;
        this._offset.value = value;
    }

    set maximum(value: number) {
        // 输入除以 20 然后向上取整
        this._maximum.value = Math.ceil(value / 20);
        // 检查是否需要调整 offset
        if (this.offset > this.maximum) this.offset = this.maximum;
    }
}

export class CategoryFilterModule<Raw extends IMrApi.TagCategoryResult & IMrFilterItem> {
    private _raw: Ref<Raw[]> = ref([]);

    // 过滤条件（你想要多少都行）
    private type = ref("");
    private header = ref("");

    constructor(type: string, header: string) {
        this.type.value = type;
        this.header.value = header;
        watch(
            this._raw,
            (list) => {
                list.forEach((it) => {
                    if (it.value === undefined) it.value = it.name;
                    if (it.label === undefined)
                        it.translate = `Main.Download/Public.Categories.${unifyString(it.name)}`;
                    if (it.ignorable === undefined) it.ignorable = true;
                    if (it.checked === undefined) it.checked = false;
                    if (it.ignored === undefined) it.ignored = false;
                });
            },
            { immediate: true, deep: true }
        );
    }

    get raw() {
        return this._raw;
    }

    // ★ 新增过滤版本（你要的 computed）
    filtered = computed<Raw[]>(() => {
        let arr = this._raw.value;

        // 过滤项目类型
        if (this.type.value) {
            arr = arr.filter((it) => it.project_type === this.type.value);
        }

        // 过滤标题
        if (this.header.value) {
            arr = arr.filter((it) => it.header.toLowerCase() === this.header.value.toLowerCase());
        }

        return arr;
    });

    async init() {
        const res = await modrinthApiAdapter.getTagCategories();
        this._raw.value = res.data as any;
    }
}

export class VersionFilterModule<Raw extends IMrApi.TagGameVersionsResult & IMrFilterItem> {
    private _raw: Ref<Raw[]> = ref([]);

    // 与原逻辑一致的过滤条件
    showAll = ref(false);
    search = ref("");

    constructor() {
        // 自动补字段（逻辑照你的）
        watch(
            this._raw,
            (list) => {
                list.forEach((it) => {
                    if (it.value === undefined) it.value = it.version;
                    if (it.label === undefined) it.label = it.version;
                    if (it.checked === undefined) it.checked = false;
                    if (it.ignored === undefined) it.ignored = false;
                });
            },
            { immediate: true, deep: true }
        );
    }

    get raw() {
        return this._raw;
    }

    // 完整等价于你的 versions computed
    filtered = computed<IMrFilterItem[]>(() => {
        let arr = this._raw.value;

        // 是否显示所有版本
        if (!this.showAll.value) {
            arr = arr.filter((it) => it.major && it.version.startsWith("1"));
        }

        // 搜索过滤
        if (this.search.value) {
            const keyword = this.search.value.toLowerCase();
            arr = arr.filter((it) => it.version.toLowerCase().includes(keyword));
        }

        return arr;
    });

    // 原逻辑的 init
    async init() {
        const res = await modrinthApiAdapter.getTagGameVersions();
        this._raw.value = res.data as any; // 保持你原本的强制塞入逻辑，watch 会补字段
    }
}

export class LoaderFilterModule<Raw extends IMrApi.TagLoaderResult & IMrFilterItem> {
    private _raw: Ref<Raw[]> = ref([]);

    // 过滤条件（你想要多少都行）
    private type = ref("");
    showAll = ref(false);

    constructor(type: string) {
        this.type.value = type;
        watch(
            this._raw,
            (list) => {
                list.forEach((it) => {
                    if (it.value === undefined) it.value = it.name;
                    if (it.label === undefined) it.translate = `Main.Download/Public.Loaders.${unifyString(it.name)}`;
                    if (it.ignorable === undefined) it.ignorable = true;
                    if (it.checked === undefined) it.checked = false;
                    if (it.ignored === undefined) it.ignored = false;
                });
            },
            { immediate: true, deep: true }
        );
    }

    get raw() {
        return this._raw;
    }

    // ★ 新增过滤版本（你要的 computed）
    filtered = computed<Raw[]>(() => {
        let arr = this._raw.value;

        // 过滤项目类型
        if (this.type.value) {
            arr = arr.filter((it) => it.supported_project_types.includes(this.type.value));
            arr = arr.filter((it) => !it.supported_project_types.includes("plugin"));
            arr = arr.filter((it) => !it.supported_project_types.includes("datapack"));
        }

        // 过滤是否展示全部
        const minimum = ["forge", "neoforge", "fabric", "quilt"];
        if (!this.showAll.value) {
            arr = arr.filter((it) => minimum.includes(it.name.toLowerCase()));
        }

        // 排序，把 forge, neoforge, fabric, quilt 四个按照顺序提到前头，其他不变
        arr.sort((a, b) => {
            const aIndex = minimum.indexOf(a.name.toLowerCase());
            const bIndex = minimum.indexOf(b.name.toLowerCase());
            if (aIndex !== -1 && bIndex !== -1) {
                return aIndex - bIndex;
            } else if (aIndex !== -1) {
                return -1;
            } else if (bIndex !== -1) {
                return 1;
            } else {
                return 0;
            }
        });

        return arr;
    });

    async init() {
        const res = await modrinthApiAdapter.getTagLoaders();
        this._raw.value = res.data as any;
    }
}
