<script setup lang="ts">
    withDefaults(defineProps<{ bgColor?: string }>(), {
        bgColor: "var(--color-base-content)",
    });
</script>

<template>
    <div
        class="card aspect-video w-full bg-base-300 text-base-content border border-base-content/25"
        :style="{ '--bg-color': bgColor, '--bgc-perc': bgColor ? '4.23%' : '1.57%' }">
        <div class="card-body">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped lang="scss">
    .card {
        --bgi-color: color-mix(in srgb, var(--bg-color) var(--bgc-perc), transparent);

        background-image:
            radial-gradient(circle at bottom left, var(--bgi-color) 35%, transparent 36%),
            radial-gradient(circle at top right, var(--bgi-color) 35%, transparent 36%);
        background-size: 4.95em 4.95em;
    }
</style>
