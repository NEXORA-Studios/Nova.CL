<script setup lang="ts">
    import { computed, ref, watch } from "vue";
    import { ToolsSkinDownloader } from "@/modules";

    const selectSource = ref<string>("None");
    const selectType = ref<string>("None");
    const inputUsername = ref<string>("");

    const availableType: Record<string, { [name: string]: string }> = {
        None: {},
        mojang: {
            skin: "Main.More/Tools.SkinDownload.Types.Skin",
            // cape: "Main.More/Tools.SkinDownload.Types.Cape",
        },
        // crafatar: {
        //     avatar: "Main.More/Tools.SkinDownload.Types.Avatar",
        //     head: "Main.More/Tools.SkinDownload.Types.Head",
        //     body: "Main.More/Tools.SkinDownload.Types.Body",
        //     skin: "Main.More/Tools.SkinDownload.Types.Skin",
        //     cape: "Main.More/Tools.SkinDownload.Types.Cape",
        // },
        minotar: {
            avatar: "Main.More/Tools.SkinDownload.Types.Avatar",
            helm: "Main.More/Tools.SkinDownload.Types.Helm",
            isometric: "Main.More/Tools.SkinDownload.Types.Isometric",
            body: "Main.More/Tools.SkinDownload.Types.Body",
            bust: "Main.More/Tools.SkinDownload.Types.Bust",
            skin: "Main.More/Tools.SkinDownload.Types.Skin",
        },
    };
    const availableTypeBind = computed(() => availableType[selectSource.value]!);
    watch(selectSource, (newValue, _oldValue) => {
        const newTypeBind = availableType[newValue]!;
        if (!Object.keys(newTypeBind).includes(selectType.value) && selectType.value !== "None") {
            selectType.value = Object.keys(newTypeBind)[0]!;
        }
    });

    async function downloadIt() {
        const downloader = new ToolsSkinDownloader();
        downloader.setMeta(selectSource.value, selectType.value, inputUsername.value);
        const uri = await downloader.getDownloadUri();
        if (!uri) {
            return;
        }
        // const savePath = await Dialogs.SaveFile({
        //     Filename: `${inputUsername.value}.png`,
        //     CanCreateDirectories: true,
        //     Filters: [
        //         {
        //             DisplayName: "皮肤文件",
        //             Pattern: "*.png",
        //         },
        //     ],
        // });
        // await DownloaderService.StartDownload(uri, savePath, {});
    }
</script>

<template>
    <div class="w-full h-full p-6">
        <div class="card card-sm w-full bg-base-100 outline outline-base-content/25">
            <div class="card-body">
                <h2 class="card-title">{{ $t("Main.More/Tools.SkinDownload.Title") }}</h2>
                <p>{{ $t("Main.More/Tools.SkinDownload.Desp") }}</p>
                <div class="join w-full">
                    <select class="join-item select outline-0! w-1/5" v-model="selectSource">
                        <option value="None" disabled selected>
                            {{ $t("Main.More/Tools.SkinDownload.SelectSource") }}
                        </option>
                        <option value="mojang">Mojang</option>
                        <!-- <option value="crafatar">Crafatar</option> -->
                        <option value="minotar">Minotar</option>
                    </select>
                    <select class="join-item select outline-0! w-1/5" v-model="selectType">
                        <option value="None" disabled selected>
                            {{
                                selectSource === "None"
                                    ? $t("Main.More/Tools.SkinDownload.SelectType.SourceFirst")
                                    : $t("Main.More/Tools.SkinDownload.SelectType.Select")
                            }}
                        </option>
                        <option
                            v-for="([key, value], index) in Object.entries(availableTypeBind)"
                            :key="index"
                            :value="key">
                            {{ $t(value) }}
                        </option>
                    </select>
                    <input
                        type="text"
                        placeholder=""
                        class="join-item input outline-0! w-3/5"
                        v-model="inputUsername" />
                    <button
                        class="join-item btn btn-success"
                        @click="downloadIt"
                        :disabled="selectSource === 'None' || selectType === 'None' || !inputUsername">
                        <i class="icon-[material-symbols--download-rounded] size-6 -m-2"></i>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
