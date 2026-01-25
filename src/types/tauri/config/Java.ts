export interface JavaEntry {
    Id: string;
    Path: string;
    Type: "jre" | "jdk";
    Version: number;
}

export interface JavaConfig {
    SelectedId: string;
    AutoSelect: boolean;
    Entry: JavaEntry[];
}
