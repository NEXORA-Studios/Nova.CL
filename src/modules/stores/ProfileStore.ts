import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { Profile } from "@/types/tauri/config/Profiles";

export const useProfileStore = defineStore("profile", () => {
    // state
    const current = ref<Profile | undefined>(undefined);

    // getter
    const currentProfile = computed(() => current.value);

    // actions
    function setProfile(profile: Profile | undefined) {
        current.value = profile;
    }

    function clearProfile() {
        current.value = undefined;
    }

    return {
        current,
        currentProfile,
        setProfile,
        clearProfile,
    };
});
