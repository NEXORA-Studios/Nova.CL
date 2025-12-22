import { ref } from "vue";
import { CategoryFilterModule, LoaderFilterModule, OffsetFilterModule, VersionFilterModule } from "@/modules";

// ---------- 全局搜索输入框 ----------
const searchString = ref("");

// ---------- Versions 过滤器 ----------
const VersionsFilter = new VersionFilterModule();

// ---------- Mod 过滤器 ----------
const ModOffsetFilter = new OffsetFilterModule();
const ModCategoriesFilter = new CategoryFilterModule("mod", "categories");
const ModLoadersFilter = new LoaderFilterModule("mod");

// ---------- Modpack 过滤器 ----------
const ModpackOffsetFilter = new OffsetFilterModule();
const ModpackCategoriesFilter = new CategoryFilterModule("modpack", "categories");
const ModpackLoadersFilter = new LoaderFilterModule("modpack");

// ---------- Datapack 过滤器 ----------
const DatapackOffsetFilter = new OffsetFilterModule();
const DatapackCategoriesFilter = new CategoryFilterModule("mod", "categories");

// ---------- Resourcepack 过滤器 ----------
const ResourcepackOffsetFilter = new OffsetFilterModule();
const ResourcepackCategoriesFilter = new CategoryFilterModule("resourcepack", "categories");
const ResourcepackResolutionFilter = new CategoryFilterModule("resourcepack", "resolutions");
const ResourcepackFeaturesFilter = new CategoryFilterModule("resourcepack", "features");

// ---------- Shader 过滤器 -----------
const ShaderOffsetFilter = new OffsetFilterModule();
const ShaderCategoriesFilter = new CategoryFilterModule("shader", "categories");
const ShaderFeaturesFilter = new CategoryFilterModule("shader", "features");
const ShaderPerformanceImpactFilter = new CategoryFilterModule("shader", "performance impact");
const ShaderLoadersFilter = new LoaderFilterModule("shader");
ShaderLoadersFilter.showAll.value = true;

// 暴露一个初始化函数，供组件调用
export const useResourceFilters = () => {
    return {
        // 全局过滤器
        searchString,
        VersionsFilter,
        // 特定过滤器
        ModOffsetFilter,
        ModCategoriesFilter,
        ModLoadersFilter,
        ModpackOffsetFilter,
        ModpackCategoriesFilter,
        ModpackLoadersFilter,
        DatapackOffsetFilter,
        DatapackCategoriesFilter,
        ResourcepackOffsetFilter,
        ResourcepackCategoriesFilter,
        ResourcepackResolutionFilter,
        ResourcepackFeaturesFilter,
        ShaderOffsetFilter,
        ShaderCategoriesFilter,
        ShaderFeaturesFilter,
        ShaderPerformanceImpactFilter,
        ShaderLoadersFilter,
    };
};
