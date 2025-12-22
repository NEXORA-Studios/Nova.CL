<template>
    <div
        :class="[
            'card w-full bg-base-200 overflow-hidden flex justify-end cursor-pointer outline outline-base-content/25',
            'transition-transform duration-250 ease-in-out hover:-translate-y-0.5 hover:scale-101',
        ]">
        <div class="w-full h-full backdrop-blur-[2px] absolute"></div>
        <span class="ml-4 mb-3 text-white font-bold z-2"><slot></slot></span>
    </div>
</template>

<style lang="scss" scoped>
    div.card {
        background-size: 100% auto;
        background-image:
            linear-gradient(
                to right,
                color-mix(in srgb, black 25%, transparent),
                color-mix(in srgb, black 25%, transparent)
            ),
            var(--nova-background-image);
        background-position-y: var(--nova-background-posY);
    }
</style>
