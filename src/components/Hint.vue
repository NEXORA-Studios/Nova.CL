<script setup lang="ts">
    defineProps<{ type: "info" | "success" | "warning" | "error" }>();
</script>

<template>
    <section>
        <div
            role="alert"
            class="alert alert-info text-[color-mix(in_srgb,var(--color-info)_70%,var(--color-base-content))] bg-[color-mix(in_srgb,var(--color-info)_30%,var(--color-base-300))] border-0 border-l-3 rounded-box"
            v-if="type === 'info'">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                class="h-6 w-6 shrink-0 stroke-current">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <div class="-translate-y-[0.75px]">
                <slot></slot>
            </div>
        </div>
        <div
            role="alert"
            class="alert alert-success text-success bg-[color-mix(in_srgb,var(--color-success)_30%,var(--color-base-300))] border-0 border-l-3 rounded-box"
            v-if="type === 'success'">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 shrink-0 stroke-current"
                fill="none"
                viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div class="-translate-y-[0.75px]">
                <slot></slot>
            </div>
        </div>
        <div
            role="alert"
            class="alert alert-warning text-warning bg-[color-mix(in_srgb,var(--color-warning)_30%,var(--color-base-300))] border-0 border-l-3 rounded-box"
            v-if="type === 'warning'">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 shrink-0 stroke-current"
                fill="none"
                viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <div class="-translate-y-[0.75px]">
                <slot></slot>
            </div>
        </div>
        <div
            role="alert"
            class="alert alert-error text-error bg-[color-mix(in_srgb,var(--color-error)_30%,var(--color-base-300))] border-0 border-l-3 rounded-box"
            v-if="type === 'error'">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 shrink-0 stroke-current"
                fill="none"
                viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div class="-translate-y-[0.75px]">
                <slot></slot>
            </div>
        </div>
    </section>
</template>
