import { ITauriTypes } from "@/types";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useAccountStore = defineStore("Account", () => {
    const AccountName = ref<string | undefined>();
    const AccountType = ref<ITauriTypes.Config.ProfileConfig["Profile"][0]["Type"] | undefined>();
    const HasAccount = ref<boolean>();

    function setAccountState(name: string, type: ITauriTypes.Config.ProfileConfig["Profile"][0]["Type"]) {
        AccountName.value = name;
        AccountType.value = type;
        HasAccount.value = !!name && !!type;
    }

    function clearAccountState() {
        AccountName.value = AccountType.value = undefined;
        HasAccount.value = false;
    }

    return {
        AccountName,
        AccountType,
        HasAccount,
        setAccountState,
        clearAccountState,
    };
});
