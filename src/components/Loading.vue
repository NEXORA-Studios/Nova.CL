<script setup lang="ts">
    import { useI18n } from "vue-i18n";

    const { t } = useI18n();
</script>

<template>
    <div class="card w-96 h-32 bg-base-100 card-md shadow-sm">
        <div class="card-body flex items-center justify-center">
            <span class="loading loading-dots loading-xl"></span>
            <span class="text-lg">{{ t("Components.Loading.Loading") }}</span>
        </div>
    </div>
</template>
