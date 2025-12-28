<script setup lang="ts">
    import { ref } from "vue";

    defineProps<{ name?: string; default?: "MHF_Steve" | "MHF_Alex"; extraClass?: string }>();

    const hideFirstLayer = ref(false);
</script>

<template>
    <div class="avatar">
        <div class="rounded relative" :class="$props.extraClass">
            <figure>
                <img class="absolute" :src="`https://minotar.net/helm/${$props.name}`" @error="hideFirstLayer = true" v-if="!hideFirstLayer" />
                <img :src="`https://minotar.net/helm/${$props.default ?? 'MHF_Steve'}`" />
            </figure>
        </div>
    </div>
</template>
