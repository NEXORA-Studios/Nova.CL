<script setup lang="ts">
    defineProps<{ name?: string; default?: "MHF_Steve" | "MHF_Alex"; extraClass?: string }>();
</script>

<template>
    <div class="avatar">
        <div class="rounded relative" :class="$props.extraClass">
            <figure>
                <img class="absolute" :src="`https://minotar.net/helm/${$props.name}`" />
                <img :src="`https://minotar.net/helm/${$props.default ?? 'MHF_Steve'}`" />
            </figure>
        </div>
    </div>
</template>
