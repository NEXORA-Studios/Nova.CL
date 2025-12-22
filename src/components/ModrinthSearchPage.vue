<script setup lang="ts">
    defineProps<{ offset: number; maximum: number }>();
    defineEmits(["previous", "next"]);
</script>

<template>
    <div class="card bg-base-100 outline-none">
        <div class="card-body">
            <div class="join">
                <button class="join-item btn" @click="$emit('previous')">«</button>
                <button class="join-item btn">
                    {{ $t("Main.Download/Public.Page", { offset: offset + 1, maximum: maximum }) }}
                </button>
                <button class="join-item btn" @click="$emit('next')">»</button>
            </div>
        </div>
    </div>
</template>
