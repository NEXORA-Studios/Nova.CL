<script setup lang="ts">
    import { useI18n } from "vue-i18n";

    const { t } = useI18n();
</script>

<template>
    <div class="text-center text-sm text-base-content/50">
        {{ t("Components.LoadingNoResult.NoResult") }}
    </div>
</template>
