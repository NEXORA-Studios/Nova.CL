<script setup lang="ts">
    import { ref } from "vue";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { GraphicalButton } from "@/components";

    const isDefaultHomeContent = ref<boolean>(true);
</script>

<template>
    <div class="w-full h-full p-6">
        <section
            id="card"
            class="grid grid-cols-3 grid-rows-1 gap-6 xl:gap-8"
            :class="{
                'h-32': isDefaultHomeContent,
                'h-12': !isDefaultHomeContent,
            }">
            <GraphicalButton
                style="
                    --nova-background-image: url(&quot;/images/MainCard_Background_1.webp&quot;);
                    --nova-background-posY: -12px;
                ">
                {{ $t("Main.Home.Cards.1") }}
            </GraphicalButton>
            <GraphicalButton
                style="
                    --nova-background-image: url(&quot;/images/MainCard_Background_2.webp&quot;);
                    --nova-background-posY: -24px;
                ">
                {{ $t("Main.Home.Cards.2") }}
            </GraphicalButton>
            <GraphicalButton
                @click="$router.push('/account')"
                style="
                    --nova-background-image: url(&quot;/images/MainCard_Background_3.webp&quot;);
                    --nova-background-posY: -24px;
                ">
                {{ $t("Main.Home.Cards.3") }}
            </GraphicalButton>
        </section>
        <div
            class="card w-full bg-base-200 p-4 mt-6 outline outline-base-content/25 rounded-lg flex flex-row! gap-8 relative">
            <img src="/images/MainCard_Image_4.webp" class="h-32 rounded-md blur-[1px]" />
            <section class="flex flex-col gap-2">
                <h1 class="card-title opacity-90">{{ $t("Main.Home.Cards.News.Title") }}</h1>
                <p class="max-w-5/6 opacity-90">{{ $t("Main.Home.Cards.News.Description") }}</p>
                <p class="text-sm opacity-50">{{ $t("Main.Home.Cards.News.Description2", { version: "1.21.11" }) }}</p>
            </section>
            <button
                class="btn btn-soft btn-secondary absolute bottom-4 right-6"
                @click="
                    openUrl(
                        `https://${$i18n.locale.startsWith('zh') ? 'zh.' : ''}minecraft.wiki/w/Java${$i18n.locale.startsWith('zh') ? '版' : '_Edition_'}1.21.11`
                    )
                ">
                {{ $t("Main.Home.Cards.News.Button") }}
            </button>
        </div>
    </div>
</template>
