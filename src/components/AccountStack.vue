<script setup lang="ts">
    import type { PanInfo } from "motion-v";
    import { Motion, useMotionValue, useTransform } from "motion-v";
    import { onBeforeMount, ref } from "vue";
    import { ITauriTypes } from "@/types";

    interface StackProps {
        className?: string;
        randomRotation?: boolean;
        sensitivity?: number;
        cardDimensions?: { width: number; height: number };
        animationConfig?: { stiffness: number; damping: number };
        sendToBackOnClick?: boolean;
        profiles: ITauriTypes.TOML.ProfileConfig["profile"];
    }

    const profiles = defineModel<ITauriTypes.TOML.ProfileConfig["profile"]>("profiles");
    const topCardIndex = defineModel<number>("top-card-index");

    function syncTopCard() {
        const top = cards.value[cards.value.length - 1];
        topCardIndex.value = top ? top.id : undefined;
    }

    const props = withDefaults(defineProps<StackProps>(), {
        randomRotation: false,
        sensitivity: 200,
        cardDimensions: () => ({ width: 208, height: 208 }),
        cardEntriesLength: () => 0,
        animationConfig: () => ({ stiffness: 260, damping: 20 }),
        sendToBackOnClick: false,
    });

    const cards = ref(profiles.value?.map((profile, index) => ({ id: index + 1, profile })) || []);
    function syncProps() {
        cards.value = profiles.value?.map((profile, index) => ({ id: index + 1, profile })) || [];
    }

    type CardState = {
        x: ReturnType<typeof useMotionValue<number>>;
        y: ReturnType<typeof useMotionValue<number>>;
        rotateX: ReturnType<typeof useMotionValue<number>>;
        rotateY: ReturnType<typeof useMotionValue<number>>;
        reset: () => void;
    };

    const cardStates = new Map<number, CardState>();

    function createCardState(): CardState {
        const x = useMotionValue(0);
        const y = useMotionValue(0);

        const rotateX = useTransform(y, [-100, 100], [60, -60]);
        const rotateY = useTransform(x, [-100, 100], [-60, 60]);

        return {
            x,
            y,
            rotateX,
            rotateY,
            reset() {
                x.set(0);
                y.set(0);
            },
        };
    }

    onBeforeMount(() => {
        cards.value.forEach((card) => {
            if (!cardStates.has(card.id)) {
                cardStates.set(card.id, createCardState());
            }
        });
        syncTopCard();
    });

    function getCardState(cardId: number): CardState {
        let state = cardStates.get(cardId);
        if (!state) {
            state = createCardState();
            cardStates.set(cardId, state);
        }
        return state;
    }

    function handleDragEnd(_: PointerEvent, info: PanInfo, cardId: number) {
        if (Math.abs(info.offset.x) > props.sensitivity || Math.abs(info.offset.y) > props.sensitivity) {
            sendToBack(cardId);
        } else {
            getCardState(cardId).reset();
        }
    }

    const sendToBack = (id: number) => {
        const newCards = [...cards.value];
        const index = newCards.findIndex((card) => card.id === id);
        const [card] = newCards.splice(index, 1);
        newCards.unshift(card);
        cards.value = newCards;
    };

    defineExpose({ syncProps });
</script>

<template>
    <div class="relative" :style="{ width: cardDimensions.width + 'px', height: cardDimensions.height + 'px', perspective: 600 }">
        <template v-for="(card, index) in cards" :key="card.id">
            <Motion
                as="div"
                class="absolute cursor-grab"
                :style="{
                    x: cardStates.get(card.id)?.x,
                    y: cardStates.get(card.id)?.y,
                    rotateX: cardStates.get(card.id)?.rotateX,
                    rotateY: cardStates.get(card.id)?.rotateY,
                }"
                drag
                :drag-constraints="{ top: 0, right: 0, bottom: 0, left: 0 }"
                :dragElastic="0.6"
                :whileTap="{ cursor: 'grabbing', scale: 1.02 }"
                :onDragEnd="(e, info) => handleDragEnd(e, info, card.id)">
                <Motion
                    as="div"
                    class="overflow-hidden"
                    @click="sendToBackOnClick && sendToBack(card.id)"
                    :animate="{
                        rotateZ: (cards.length - index - 1) * 4 + (randomRotation ? Math.random() * 10 - 5 : 0),
                        scale: 1 + index * 0.06 - cards.length * 0.06,
                        transformOrigin: '90% 90%',
                    }"
                    :initial="false"
                    :transition="{
                        type: 'spring',
                        stiffness: animationConfig.stiffness,
                        damping: animationConfig.damping,
                    }"
                    :style="{
                        width: cardDimensions.width + 'px',
                        height: cardDimensions.height + 'px',
                    }">
                    <slot :card="card.profile" :index="index" :isTop="index === cards.length - 1"></slot>
                </Motion>
            </Motion>
        </template>
    </div>
</template>
