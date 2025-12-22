<script setup lang="ts">
    import { ELaunchStatus } from "@/utils";
    import { ref } from "vue";
    import { useI18n } from "vue-i18n";

    const { locale } = useI18n();
    const status = ref<ELaunchStatus>(ELaunchStatus.Downloaded);
    const progress = ref<number>(0);
</script>

<template>
    <footer style="grid-area: footer" class="bg-base-300/75 px-8 flex items-center">
        <section class="w-1/2 flex flex-col gap-0.5">
            <div class="flex text-sm justify-between">
                <p class="opacity-50">{{ $t(`Footer.Status.${status}`) }}</p>
                <p class="opacity-90 max-w-64 truncate">Create: Above & Beyond</p>
            </div>
            <progress
                class="progress w-full h-1"
                :class="{
                    'progress-success': [ELaunchStatus.Downloaded, ELaunchStatus.Launched].includes(status),
                    'progress-warning': status === ELaunchStatus.Downloading,
                    'progress-info': status === ELaunchStatus.Launching,
                    'progress-error': status === ELaunchStatus.Failed,
                }"
                :value="status === ELaunchStatus.Downloaded ? 100 : progress"
                max="100"></progress>
        </section>
        <section class="flex gap-2 ml-auto mr-12">
            <button class="btn btn-primary">
                <span :class="`text-${locale === 'en-US' ? 'xs' : 'sm'}`">{{ $t("Footer.LaunchGame") }}</span>
            </button>
            <div class="fab translate-y-1">
                <!-- a focusable div with tabindex is necessary to work on all browsers. role="button" is necessary for accessibility -->
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary">
                    <i class="icon-[material-symbols--action-key-outline-rounded]"></i>
                </div>

                <!-- buttons that show up when FAB is open -->
                <div class="tooltip tooltip-left" :data-tip="$t('Footer.Tooltip.KillAllMCInstance')">
                    <button class="btn btn-lg btn-circle btn-error aspect-square">
                        <i
                            class="icon-[material-symbols--sentiment-extremely-dissatisfied-outline-rounded] size-7 -m-3"></i>
                    </button>
                </div>
                <div
                    class="tooltip tooltip-left"
                    :data-tip="
                        $t('Footer.Tooltip.Music.Template', {
                            status: $t('Footer.Tooltip.Music.Pause'),
                            name: 'Shake It Off.mp3',
                            flip_status: $t('Footer.Tooltip.Music.Play'),
                        })
                    ">
                    <button class="btn btn-lg btn-circle btn-secondary aspect-square" @contextmenu.prevent="">
                        <i class="icon-[material-symbols--play-circle-outline-rounded] size-7 -m-3"></i>
                    </button>
                </div>
            </div>
        </section>
    </footer>
</template>
