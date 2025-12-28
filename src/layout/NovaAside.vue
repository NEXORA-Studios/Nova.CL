<script setup lang="ts">
    import { Avatar } from "@/components";
    import { useAccountStore } from "@/modules";
    import { RuntimeCache } from "@/utils";
    import { useTheme } from "@/composables";

    const accountStore = useAccountStore();
    const { matchTheme } = useTheme();
</script>

<template>
    <aside style="grid-area: aside" class="bg-base-300/75 border-r border-base-content/25 flex flex-col px-8 pt-4">
        <section class="h-14 flex items-center gap-4 translate-y-0.75">
            <figure class="scale-110 -translate-x-2">
                <img src="/logo.webp" alt="logo" v-if="matchTheme('light')" />
                <img src="/logodark.webp" alt="logo" v-if="matchTheme('dark')" />
            </figure>
            <!-- <i class="icon-[octicon--rocket-16] size-6"></i> -->
            <span class="text-lg font-bold ml-5 -translate-x-9 translate-y-px">Nova.CL</span>
        </section>
        <section class="mt-12 flex flex-col gap-2">
            <button
                class="flex justify-start items-center btn btn-primary"
                :class="{ 'btn-ghost btn-soft': $route.path !== '/' }"
                @click="$router.push('/')">
                <i class="icon-[material-symbols--home-outline-rounded] size-5"></i>
                <span class="font-medium">{{ $t("Aside.MenuButton.Home") }}</span>
            </button>
            <button
                class="flex justify-start items-center btn btn-primary"
                :class="{ 'btn-ghost btn-soft': !$route.path.startsWith('/download') }"
                @click="$router.push(RuntimeCache.GetWithDefault('aside.download.path', '/download/minecraft'))">
                <i class="icon-[material-symbols--box-add-outline-rounded] size-5"></i>
                <span class="font-medium">{{ $t("Aside.MenuButton.Download") }}</span>
            </button>
            <button
                class="flex justify-start items-center btn btn-primary"
                :class="{ 'btn-ghost btn-soft': !$route.path.startsWith('/setting') }"
                @click="$router.push(RuntimeCache.GetWithDefault('aside.setting.path', '/setting/launch'))">
                <i class="icon-[material-symbols--settings-outline-rounded] size-5"></i>
                <span class="font-medium">{{ $t("Aside.MenuButton.Settings") }}</span>
            </button>
            <button
                class="flex justify-start items-center btn btn-primary"
                :class="{ 'btn-ghost btn-soft': !$route.path.startsWith('/more') }"
                @click="$router.push(RuntimeCache.GetWithDefault('aside.more.path', '/more/about'))">
                <i class="icon-[material-symbols--grid-view-outline-rounded] size-5"></i>
                <span class="font-medium">{{ $t("Aside.MenuButton.More") }}</span>
            </button>
        </section>
        <section class="h-14 mt-auto mb-4 -mx-2 px-2 pl-4 grid grid-cols-5 items-center gap-4 btn btn-ghost" @click="$router.push('/profile')">
            <section class="size-10 aspect-square">
                <img src="/images/barrier.png" class="size-10 -ml-4" v-if="!accountStore.HasAccount" />
                <Avatar :name="accountStore.AccountName" extra-class="size-10" v-else-if="accountStore.AccountType === 'msa'" />
                <Avatar name="MHF_Steve" extra-class="size-10" v-else />
            </section>
            <div class="flex flex-col col-span-4" v-if="!accountStore.HasAccount">
                <span class="opacity-90">{{ $t("Aside.NoProfile.__Title__") }}</span>
                <span class="text-xs opacity-50">{{ $t("Aside.NoProfile.__Hint__") }}</span>
            </div>
            <div class="w-full col-span-4 flex flex-col" v-else>
                <span class="opacity-90">{{ accountStore.AccountName }}</span>
                <span class="text-xs opacity-50 font-normal">
                    {{ accountStore.AccountType !== undefined ? $t(`Aside.AccountType.${accountStore.AccountType}`) : "" }}
                </span>
            </div>
        </section>
    </aside>
</template>
