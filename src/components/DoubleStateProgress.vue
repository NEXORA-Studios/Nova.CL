<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useI18n } from "vue-i18n";
    import { useTimer } from "@/composables";

    const props = defineProps<{
        current: number;
        use: number;
        max: number;
    }>();

    const { t } = useI18n();
    const { onShortInterval } = useTimer();

    // const progressbar = ref<HTMLDivElement>();
    const state1 = ref<number>();
    const state2 = ref<number>();
    const state2Max = ref<boolean>(false);

    function update() {
        state1.value = (props.current / props.max) * 100;
        const _state2 = (props.use / 1024 / props.max) * 100;
        if (state1.value + _state2 <= 100) {
            state2.value = _state2;
            state2Max.value = false;
        } else {
            state2.value = 100 - state1.value;
            state2Max.value = true;
        }
    }

    onShortInterval(update);
    onMounted(update);
</script>

<template>
    <div role="progressbar" ref="progressbar" class="progressbar">
        <div
            class="progressbar__state progressbar__state1"
            :style="{ width: state1 + '%' }"
            :data-content-above="t('Main.Setting.Launch.RAM.Used/Installed')"
            :data-content-bottom="`${Math.round($props.current * 10) / 10} GB / ${Math.round($props.max * 10) / 10} GB`"></div>
        <div
            class="progressbar__state progressbar__state2"
            :class="{ progressbar__state2_max: state2Max }"
            :style="{ width: state2 + '%' }"
            :data-content-above="t('Main.Setting.Launch.RAM.Distribute')"
            :data-content-bottom="`${Math.round(($props.use / 1024) * 10) / 10} GB${state2Max ? t('Main.Setting.Launch.RAM.Available', { available: Math.round(($props.max - $props.current) * 10) / 10 }) : ''}`"></div>
    </div>
</template>

<style lang="scss" scoped>
    .progressbar {
        width: 100%;
        height: calc(var(--spacing) * 2);
        border-radius: var(--radius-box);
        display: flex;
        background-color: color-mix(in srgb, var(--color-primary), var(--color-base-300) 70%);
        margin: calc(var(--spacing) * 6) 0 calc(var(--spacing) * 6) 0;

        .progressbar__state {
            position: relative;
            height: 100%;
            transition: width 0.3s ease-in-out;
            &::before {
                content: attr(data-content-above);
                position: absolute;
                bottom: calc(var(--spacing) * 3);
                opacity: 50%;
                font-size: var(--text-xs);
                line-height: var(--tw-leading, var(--text-xs--line-height));
                font-family: "Embedded Maple Mono";
                width: calc(var(--spacing) * 60);
            }
            &::after {
                content: attr(data-content-bottom);
                position: absolute;
                top: calc(var(--spacing) * 2.5);
                font-size: var(--text-lg);
                line-height: var(--tw-leading, var(--text-lg--line-height));
                font-family: "Embedded Maple Mono";
                width: calc(var(--spacing) * 60);
            }
        }

        .progressbar__state1 {
            background-color: color-mix(in srgb, var(--color-primary), var(--color-base-300) 10%);
            border-radius: var(--radius-box) 0 0 var(--radius-box);
        }

        .progressbar__state2 {
            background-color: color-mix(in srgb, var(--color-primary), var(--color-base-300) 50%);
            border-radius: 0 var(--radius-box) var(--radius-box) 0;

            &:not(.progressbar__state2_max) {
                &::before,
                &::after {
                    left: 1px;
                }
            }

            &.progressbar__state2_max {
                &::before {
                    right: 1px;
                    text-align: right;
                }
                &:after {
                    right: calc(var(--spacing) * -2.5);
                    text-align: right;
                }
            }
        }
    }
</style>
