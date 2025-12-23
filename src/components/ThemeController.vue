<script setup lang="ts">
    import { ref } from "vue";
    import { EventBus } from "@/modules/service/EventBus";

    interface IOption {
        text: string;
        value: string;
    }
    const ThemeOptions: IOption[] = [
        // 自动
        { text: "自动（跟随系统）", value: "auto" },
        { text: "Light", value: "light" },
        { text: "Dark", value: "dark" },
        // { text: "Cupcake", value: "cupcake" },
        // { text: "Bumblebee", value: "bumblebee" },
        // { text: "Emerald", value: "emerald" },
        // { text: "Corporate", value: "corporate" },
        // { text: "Synthwave", value: "synthwave" },
        // { text: "Retro", value: "retro" },
        // { text: "Cyberpunk", value: "cyberpunk" },
        // { text: "Valentine", value: "valentine" },
        // { text: "Halloween", value: "halloween" },
        // { text: "Garden", value: "garden" },
        // { text: "Forest", value: "forest" },
        // { text: "Aqua", value: "aqua" },
        // { text: "Lofi", value: "lofi" },
        // { text: "Pastel", value: "pastel" },
        // { text: "Fantasy", value: "fantasy" },
        // { text: "Wireframe", value: "wireframe" },
        // { text: "Luxury", value: "luxury" },
        // { text: "Dracula", value: "dracula" },
        // { text: "CMYK", value: "cmyk" },
        // { text: "Business", value: "business" },
        // { text: "Lemonade", value: "lemonade" },
        // { text: "Night", value: "night" },
        // { text: "Coffee", value: "coffee" },
        // { text: "Winter", value: "winter" },
        // { text: "Dim", value: "dim" },
        // { text: "Nord", value: "nord" },
        // { text: "Sunset", value: "sunset" },
        // { text: "Caramellatte", value: "caramellatte" },
        // { text: "Silk", value: "silk" },
    ];

    const theme = ref<string>("auto");
    EventBus.on("theme:change", (e) => {
        theme.value = e;
        // 应用主题
        if (e === "auto") {
            document.documentElement.removeAttribute("data-theme");
        } else {
            document.documentElement.setAttribute("data-theme", e);
        }
    });
</script>

<template>
    <section data-region-for="theme-controller" class="hidden">
        <input
            v-for="option in ThemeOptions"
            type="checkbox"
            :name="option.text"
            :value="option.value"
            class="toggle theme-controller"
            :checked="theme === option.value" />
    </section>
</template>
