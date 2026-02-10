<template>
    <div ref="root" class="scroll-area" :style="style">
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
    import { computed, StyleValue } from "vue";

    const props = defineProps<{
        height?: string | number;
        maxHeight?: string | number;
    }>();

    const normalize = (v?: string | number) => (typeof v === "number" ? `${v}px` : v);

    const style = computed(
        () =>
            ({
                height: normalize(props.height),
                maxHeight: normalize(props.maxHeight),
                overflowY: "auto",
                overscrollBehavior: "contain", // 阻断滚动穿透
            }) as StyleValue
    );
</script>

<style scoped>
    .scroll-area {
        width: 100%;
    }
</style>
