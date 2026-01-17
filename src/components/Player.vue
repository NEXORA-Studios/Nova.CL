<script setup lang="ts">
    import * as THREE from "three";
    import { OrbitControls } from "three/addons/controls/OrbitControls.js";
    import { ref, onMounted, watch } from "vue";
    import { PlayerAnimator, glTFDatasets, PlayerPose, PlayerGltfBuilder, CapeAnimator, CapeGltfBuilder } from "@/modules/gltf";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { EventBus } from "@/modules";
    import { useTheme } from "@/composables";

    const { matchTheme } = useTheme();

    const props = withDefaults(
        defineProps<{
            /** 预期的类型："wide" 或 "slim" */
            type?: string;
            skinUrl?: string;
            capeUrl?: string;
        }>(),
        {
            type: "slim",
            skinUrl: "https://minotar.net/skin/MHF_Steve",
        }
    );
    const container = ref<HTMLDivElement>();

    const scene = new THREE.Scene();

    const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });

    let controls: OrbitControls;
    let playerAnimator: PlayerAnimator | null = null;
    let capeAnimator: CapeAnimator | null = null;

    // 预计算挥手姿势（只计算一次）
    const wavePose = PlayerPose.diff(
        props.type === "wide" ? new glTFDatasets.WideIdleDataset().getNodes() : new glTFDatasets.SlimIdleDataset().getNodes(),
        props.type === "wide" ? new glTFDatasets.WideActionDataset().getNodes() : new glTFDatasets.SlimActionDataset().getNodes()
    );

    const idlePose = new PlayerPose("idle", []);

    let isWavingSequence = false;

    async function cleanSence() {
        if (playerAnimator) {
            playerAnimator = null;
        }
        const filteredChildren = scene.children;
        scene.remove(...filteredChildren);
    }

    async function reload() {
        await cleanSence();
        await loadScene();
    }

    watch(() => [props.skinUrl, props.capeUrl], reload);

    async function loadScene() {
        const brightness = 1;
        scene.add(new THREE.AmbientLight(0xffffff, brightness));
        const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
        directionalLight.intensity = 2;
        directionalLight.position.set(3, 5, 3);
        scene.add(directionalLight);

        const dataset = props.type === "wide" ? new glTFDatasets.WideIdleDataset() : new glTFDatasets.SlimIdleDataset();

        const playerBuilder = new PlayerGltfBuilder().customUrl(props.skinUrl).dataset(dataset);
        playerAnimator = await playerBuilder.loadIntoScene(scene);

        let hasCape = false;

        if (props.capeUrl) {
            const capeBuilder = new CapeGltfBuilder().customUrl(props.capeUrl);
            capeAnimator = await capeBuilder.loadIntoScene(scene);
            hasCape = true;
        }

        // 找到玩家模型（通常是最后一个添加的 Group）
        const player =
            scene.children.find((c) => c.type === "Group" && c.children.length > 10) ||
            scene.children[scene.children.length - (hasCape ? 2 : 1)];

        if (!player) {
            console.tError({ category: "Player Render", message: "未找到玩家模型" });
            return;
        }

        // 计算包围盒并居中
        const box = new THREE.Box3().setFromObject(player);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());

        player.position.sub(center);
        player.rotation.y = Math.PI; // 正面朝相机

        // 关键：放大模型！
        const scale = 0.4;
        player.scale.set(scale, scale, scale);
        player.position.y -= size.y * scale * 3.75;

        // 调整相机
        camera.position.set(0, size.y * scale * 0.75, size.z * scale * 3);
        camera.lookAt(0, size.y * scale * 0.5, 0);
        controls.target.set(0, size.y * scale * 0.5, 0);
        controls.update();
    }

    async function startWaveSequence() {
        if (isWavingSequence || !playerAnimator) return;

        isWavingSequence = true;

        const doOneWave = async () => {
            // 挥手 → 到达顶点停留 300ms → 回正 → 短暂停顿 → 下一轮
            playerAnimator!.transitionTo(wavePose, 400);
            await sleep(400 + 1000); // 挥手时间 + 停留

            playerAnimator!.transitionTo(idlePose, 400);
            await sleep(400 + 300); // 回正时间 + 间隔

            isWavingSequence = false;
        };

        doOneWave();
    }

    // 简单的 sleep 工具
    function sleep(ms: number): Promise<void> {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    // 点击重新触发行挥手序列
    function onStartWave() {
        if (!playerAnimator || playerAnimator.isPlaying) return;
        startWaveSequence();
    }
    defineExpose({
        onStartWave,
    });

    let frame = 0;
    function randInt(min: number, max: number): number {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    onMounted(async () => {
        if (!container.value) return;

        const width = container.value.clientWidth;
        const height = container.value.clientHeight;
        camera.aspect = width / height;
        camera.updateProjectionMatrix();

        renderer.setSize(width, height);
        renderer.setPixelRatio(window.devicePixelRatio);
        container.value.appendChild(renderer.domElement);

        controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.enablePan = false;
        controls.minDistance = 1;
        controls.maxDistance = 10;

        await loadScene();

        // 首次自动演示挥手
        startWaveSequence();

        // 渲染循环
        function animate() {
            requestAnimationFrame(animate);

            controls.update();
            playerAnimator?.update();
            capeAnimator?.update();

            // 缓慢自转展示模型
            if (!isWavingSequence) scene.rotation.y += 0.005;

            if (frame % 100 === 0) {
                capeAnimator?.transitionTo(randInt(0, 4), 400);
            }

            renderer.render(scene, camera);

            frame++;
        }
        animate();
        onResize();
        EventBus.on("theme:change", reload);
    });

    // 窗口大小变化
    const onResize = () => {
        if (!container.value) return console.tWarn({ category: "Player Render", message: "Resize failed" });

        // 直接用窗口尺寸（减去左侧固定 406px + gap）
        const totalWidth = window.innerWidth;
        const totalHeight = window.innerHeight;

        // 简单粗暴：右侧占剩余空间
        const width = totalWidth - (175 + 24 * 2 + 410 + 16 + 72);
        const height = totalHeight - (56 + 24 * 2 + 48 + 16 + 64);

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
        renderer.setSize(width, height);
    };
    getCurrentWindow().onResized(onResize);
</script>

<template>
    <div ref="container" class="w-full h-full" />
</template>

<style lang="scss" scoped>
    canvas {
        display: block;
        width: 100% !important;
        height: 100% !important;
    }
</style>
