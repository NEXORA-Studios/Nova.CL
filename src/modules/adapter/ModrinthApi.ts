import { Requester } from "@/modules/service/Requster";
import type { IMrApi, IMrFilterItem } from "@/types";
import { TagGameVersionsResult, TagCategoryResult, TagLoaderResult } from "@/types/MrApi";
import { Ref } from "vue";

type VersionFilter = (TagGameVersionsResult & IMrFilterItem)[];
type CategoryFilter = (TagCategoryResult & IMrFilterItem)[];
type LoaderFilter = (TagLoaderResult & IMrFilterItem)[];

class ModrinthApiAdapter {
    private requester: Requester;

    constructor(requester: Requester) {
        this.requester = requester;
    }

    private makeSearchModParams(key: string, item: IMrFilterItem[]) {
        let string: string[] = [];
        item.forEach((i) => {
            if (i.checked) {
                string.push(`"${key}=${i.value}"`);
            }
            if (i.ignored) {
                string.push(`"${key}!=${i.value}"`);
            }
        });
        return string.length > 0 ? `[${string.join(",")}],` : "";
    }

    async searchMod(
        query: string,
        versionFilter: Ref<VersionFilter, VersionFilter>,
        categoryFilter: Ref<CategoryFilter, CategoryFilter>,
        loaderFilter: Ref<LoaderFilter, LoaderFilter>,
        limit: number = 20,
        offset: number = 0
    ): Promise<IMrApi.ProjectSearchResult> {
        const staticParams = `["project_type:mod"],["client_side:optional","client_side:required"],["server_side:optional","server_side:unsupported"]`;
        const versionParams = this.makeSearchModParams("versions", versionFilter.value);
        const categoryParams = this.makeSearchModParams("categories", categoryFilter.value);
        const loaderParams = this.makeSearchModParams("loaders", loaderFilter.value);
        const facets = `[${versionParams}${categoryParams}${loaderParams}${staticParams}]`;
        const params = `query=${query}&limit=${limit}&offset=${offset}&facets=${facets}`;
        return (await this.requester.get<IMrApi.ProjectSearchResult>(`/search?${params}`)).data;
    }

    async searchModpack(
        query: string,
        versionFilter: Ref<VersionFilter, VersionFilter>,
        categoryFilter: Ref<CategoryFilter, CategoryFilter>,
        loaderFilter: Ref<LoaderFilter, LoaderFilter>,
        limit: number = 20,
        offset: number = 0
    ): Promise<IMrApi.ProjectSearchResult> {
        const staticParams = `["project_type:modpack"],["client_side:optional","client_side:required"],["server_side:optional","server_side:unsupported"]`;
        const versionParams = this.makeSearchModParams("versions", versionFilter.value);
        const categoryParams = this.makeSearchModParams("categories", categoryFilter.value);
        const loaderParams = this.makeSearchModParams("loaders", loaderFilter.value);
        const facets = `[${versionParams}${categoryParams}${loaderParams}${staticParams}]`;
        const params = `query=${query}&limit=${limit}&offset=${offset}&facets=${facets}`;
        return (await this.requester.get<IMrApi.ProjectSearchResult>(`/search?${params}`)).data;
    }

    async searchDatapack(
        query: string,
        versionFilter: Ref<VersionFilter, VersionFilter>,
        categoryFilter: Ref<CategoryFilter, CategoryFilter>,
        limit: number = 20,
        offset: number = 0
    ): Promise<IMrApi.ProjectSearchResult> {
        const staticParams = `["project_type:datapack"]`;
        const versionParams = this.makeSearchModParams("versions", versionFilter.value);
        const categoryParams = this.makeSearchModParams("categories", categoryFilter.value);
        const facets = `[${versionParams}${categoryParams}${staticParams}]`;
        const params = `query=${query}&limit=${limit}&offset=${offset}&facets=${facets}`;
        return (await this.requester.get<IMrApi.ProjectSearchResult>(`/search?${params}`)).data;
    }

    async searchResourcepack(
        query: string,
        versionFilter: Ref<VersionFilter, VersionFilter>,
        categoryFilter: Ref<CategoryFilter, CategoryFilter>,
        resolutionFilter: Ref<CategoryFilter, CategoryFilter>,
        featuresFilter: Ref<CategoryFilter, CategoryFilter>,
        limit: number = 20,
        offset: number = 0
    ): Promise<IMrApi.ProjectSearchResult> {
        const staticParams = `["project_type:resourcepack"]`;
        const versionParams = this.makeSearchModParams("versions", versionFilter.value);
        const categoryParams = this.makeSearchModParams("categories", categoryFilter.value);
        const resolutionParams = this.makeSearchModParams("categories", resolutionFilter.value);
        const featuresParams = this.makeSearchModParams("categories", featuresFilter.value);
        const facets = `[${versionParams}${categoryParams}${resolutionParams}${featuresParams}${staticParams}]`;
        const params = `query=${query}&limit=${limit}&offset=${offset}&facets=${facets}`;
        return (await this.requester.get<IMrApi.ProjectSearchResult>(`/search?${params}`)).data;
    }

    async searchShader(
        query: string,
        versionFilter: Ref<VersionFilter, VersionFilter>,
        categoryFilter: Ref<CategoryFilter, CategoryFilter>,
        performanceImpactFilter: Ref<CategoryFilter, CategoryFilter>,
        loaderFilter: Ref<LoaderFilter, LoaderFilter>,
        limit: number = 20,
        offset: number = 0
    ): Promise<IMrApi.ProjectSearchResult> {
        const staticParams = `["project_type:shader"]`;
        const versionParams = this.makeSearchModParams("versions", versionFilter.value);
        const categoryParams = this.makeSearchModParams("categories", categoryFilter.value);
        const performanceImpactParams = this.makeSearchModParams("categories", performanceImpactFilter.value);
        const loaderParams = this.makeSearchModParams("loaders", loaderFilter.value);
        const facets = `[${versionParams}${categoryParams}${performanceImpactParams}${loaderParams}${staticParams}]`;
        const params = `query=${query}&limit=${limit}&offset=${offset}&facets=${facets}`;
        return (await this.requester.get<IMrApi.ProjectSearchResult>(`/search?${params}`)).data;
    }

    async getTagGameVersions() {
        return await this.requester.get<IMrApi.TagGameVersionsResult[]>("/tag/game_version");
    }

    async getTagCategories() {
        return await this.requester.get<IMrApi.TagCategoryResult[]>("/tag/category");
    }

    async getTagLoaders() {
        return await this.requester.get<IMrApi.TagLoaderResult[]>("/tag/loader");
    }
}

export const modrinthApiAdapter = new ModrinthApiAdapter(new Requester({ baseUrl: "https://api.modrinth.com/v2" }));
