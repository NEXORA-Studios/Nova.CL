export interface TagGameVersionsResult {
    version: string;
    version_type: "release" | "snapshot" | "alpha" | "beta";
    date: string;
    major: boolean;
}

export interface TagCategoryResult {
    icon: string;
    name: string;
    project_type: string;
    header: string;
}

export interface TagLoaderResult {
    icon: string;
    name: string;
    supported_project_types: string[];
}

export interface ProjectSearchResult {
    hits: SingleProjectResult[];
    offset: number;
    limit: number;
    total_hits: number;
}

export interface SingleProjectResult {
    project_id: string;
    project_type: string;
    slug: string;
    author: string;
    title: string;
    description: string;

    categories: string[];
    display_categories: string[];
    versions: string[];

    downloads: number;
    follows: number;

    icon_url: string;
    date_created: string; // ISO 8601
    date_modified: string; // ISO 8601

    latest_version: string;
    license: string;

    client_side: "required" | "optional" | "unsupported";
    server_side: "required" | "optional" | "unsupported";

    gallery: string[];
    featured_gallery: string | null;

    color: number;
}
