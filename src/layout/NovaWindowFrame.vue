<template>
    <div class="wrapper">
        <div class="main-container">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
    .wrapper {
        --wallpaper-url: url("/images/Minecraft_Fall_Drop_Campaign_Key_Art_DotNet_Downloadable_Wallpaper_1920x1080.webp");

        background-image: var(--wallpaper-url);
        width: 100vw;
        height: 100vh;
    }

    .main-container {
        width: 100%;
        height: 100%;
        background-color: color-mix(in srgb, var(--color-base-300) 75%, transparent);
        backdrop-filter: blur(8px);
        display: grid;
        grid-template-columns: 240px 1fr;
        grid-template-rows: 56px 1fr 72px;
        grid-template-areas:
            "aside header"
            "aside main"
            "aside footer";
    }
</style>
