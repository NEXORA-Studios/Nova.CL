import { EAccountType } from "@/utils"
import { defineStore } from "pinia"
import { ref } from "vue"

export const useAccountStore = defineStore("Account", () => {
    const AccountName = ref<string | undefined>("LingyunAwA")
    const AccountType = ref<EAccountType | undefined>(EAccountType.Online)

    function setAccountState(name: string, type: EAccountType) {
        AccountName.value = name
        AccountType.value = type
    }

    function clearAccountState() {
        AccountName.value = AccountType.value = undefined
    }

    return {
        AccountName,
        AccountType,
        setAccountState,
        clearAccountState,
    }
})
