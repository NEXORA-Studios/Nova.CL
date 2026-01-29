<script setup lang="ts">
    import { Avatar, BankCard } from ".";
    import { ITauriTypes } from "@/types";
    import { insertEvery } from "@/utils";

    defineProps<{ profile: ITauriTypes.Config.ProfileConfig["Profile"][number] }>();
    const TypeMapper = {
        msa: ["Components.AccountCard.AccountType.Msa", "var(--color-success)"],
        legacy: ["Components.AccountCard.AccountType.Legacy", "var(--color-warning)"],
        yggdrasil: ["Components.AccountCard.AccountType.Yggdrasil", "var(--color-error)"],
    };
</script>

<template>
    <BankCard :bgColor="TypeMapper[profile.Type][1] ? TypeMapper[profile.Type][1] : 'var(--color-base-content)'">
        <div class="flex justify-between mb-10">
            <div class="font-bold text-base">{{ $t(TypeMapper[profile.Type][0]) }}</div>
            <div class="opacity-75">
                <Avatar :name="profile.Type === 'msa' ? profile.Name : ''" extra-class="size-12" />
            </div>
        </div>
        <div class="text-lg mb-4 opacity-40">{{ insertEvery(profile.Uuid.toLowerCase().replace(/-/g, ""), " ") }}</div>
        <div class="flex justify-between -translate-y-1">
            <div>
                <div class="text-xs opacity-20">{{ $t("Components.AccountCard.Player") }}</div>
                <div>{{ profile.Name }}</div>
            </div>
            <div>
                <div class="text-xs opacity-20">
                    <span v-if="profile.Type === 'msa'">{{ $t("Components.AccountCard.Expire") }}</span>
                </div>
                <div>
                    <span v-if="profile.Type === 'msa'">{{ new Date(profile.MsaExpiresAt).toLocaleString() }}</span>
                </div>
            </div>
        </div>
    </BankCard>
</template>
