<script setup lang="ts">
    defineProps<{
        ignorable?: boolean;
    }>();

    // 分别接收两个 v-model
    const checked = defineModel<boolean>("checked", { required: true });
    const ignored = defineModel<boolean>("ignored", { default: false });

    // 点击主按钮的逻辑：三态切换
    const toggleMain = () => {
        if (ignored.value) {
            ignored.value = false;
        } else {
            checked.value = !checked.value;
        }
    };

    // 点击右侧小×按钮（仅在未选中且可忽略时显示）
    const toggleIgnore = () => {
        ignored.value = !ignored.value;
        if (ignored.value) {
            checked.value = false; // 忽略时强制取消选中
        }
    };
</script>

<template>
    <div class="w-full grid grid-cols-[1fr_32px] grid-rows-1 gap-1">
        <button
            class="btn btn-sm rounded-full justify-start! group transition-all outline-none"
            :class="{
                'dark:btn-neutral dark:bg-neutral/10 dark:hover:bg-neutral/35 dark:text-base-content':
                    !checked && !ignored,
                'bg-success/15 dark:bg-success/10 hover:bg-success/25 dark:hover:bg-success/35 border-success/50 dark:border-success/25':
                    checked,
                'bg-error/15 dark:bg-error/10 hover:bg-error/25 dark:hover:bg-error/35 border-error/50 dark:border-error/25':
                    ignored,
                'col-span-2': checked || ignored || !$props.ignorable,
            }"
            @click="toggleMain">
            <slot></slot>
            <svg
                v-if="checked || (!checked && !ignored)"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                viewBox="0 0 24 24"
                class="check-icon ml-auto size-4 stroke-success -mr-1"
                :class="{
                    'transition-opacity dark:opacity-10 group-hover:opacity-100': !(checked || ignored),
                }"
                aria-hidden="true">
                <path d="M20 6 9 17l-5-5"></path>
            </svg>
            <svg
                v-if="ignored"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                class="size-4 -mx-1 stroke-error"
                :class="{
                    'transition-opacity opacity-50 dark:opacity-10 group-hover:opacity-100': !(checked || ignored),
                    'ml-auto': ignored,
                }"
                viewBox="0 0 24 24"
                aria-hidden="true">
                <circle cx="12" cy="12" r="10"></circle>
                <path d="m4.9 4.9 14.2 14.2"></path>
            </svg>
        </button>
        <button
            class="btn btn-sm w-full rounded-full justify-start! group transition-all dark:btn-neutral dark:bg-neutral/10 dark:hover:bg-neutral/35 outline-none"
            @click="toggleIgnore"
            v-if="!(checked || ignored) && $props.ignorable">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                class="size-4 -mx-1 stroke-error transition-opacity dark:opacity-10 group-hover:opacity-100"
                viewBox="0 0 24 24"
                aria-hidden="true">
                <circle cx="12" cy="12" r="10"></circle>
                <path d="m4.9 4.9 14.2 14.2"></path>
            </svg>
        </button>
    </div>
</template>
