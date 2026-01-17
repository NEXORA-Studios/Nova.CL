import * as THREE from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { glTFDatasets, PlayerAnimator, IllegalStateException } from ".";

export class PlayerGltfBuilder {
    private _customUrl: string | undefined = undefined;
    private _dataset: glTFDatasets.GltfDataset | null = null;

    customUrl(url?: string): this {
        this._customUrl = url;
        return this;
    }

    dataset(dataset: glTFDatasets.GltfDataset): this {
        this._dataset = dataset;
        return this;
    }

    private buildGltfJson(): Record<string, any> {
        if (!this._dataset) throw new IllegalStateException("Dataset not set");
        if (!this._customUrl) throw new IllegalStateException("Custom URL not set");

        // 永远使用 idle 姿势作为基础（getNodes() 返回 idle）
        const nodes = this._dataset.getNodes();

        return {
            asset: { version: "2.0", generator: "MCPlayerGltfBuilder" },
            scenes: [{ nodes: [12, 15, 18] }],
            scene: 0,
            nodes,
            bufferViews: [
                { buffer: 0, byteOffset: 0, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 288, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 576, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 768, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 840, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 1128, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 1416, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 1608, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 1680, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 1968, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 2256, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 2448, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 2520, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 2808, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 3096, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 3288, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 3360, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 3648, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 3936, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 4128, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 4200, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 4488, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 4776, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 4968, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 5040, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 5328, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 5616, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 5808, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 5880, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 6168, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 6456, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 6648, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 6720, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 7008, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 7296, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 7488, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 7560, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 7848, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 8136, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 8328, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 8400, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 8688, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 8976, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 9168, byteLength: 72, target: 34963 },
                { buffer: 0, byteOffset: 9240, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 9528, byteLength: 288, target: 34962, byteStride: 12 },
                { buffer: 0, byteOffset: 9816, byteLength: 192, target: 34962, byteStride: 8 },
                { buffer: 0, byteOffset: 10008, byteLength: 72, target: 34963 },
            ],
            buffers: this._dataset.getBuffers(),
            accessors: this._dataset.getAccessors(),
            materials: [
                {
                    pbrMetallicRoughness: {
                        metallicFactor: 0,
                        roughnessFactor: 1,
                        baseColorTexture: { index: 0 },
                    },
                    alphaMode: "MASK",
                    alphaCutoff: 0.05,
                    doubleSided: true,
                },
            ],
            textures: [{ sampler: 0, source: 0 }],
            samplers: [{ magFilter: 9728, minFilter: 9728, wrapS: 33071, wrapT: 33071 }],
            images: [
                {
                    mimeType: "image/png",
                    uri: this._customUrl,
                },
            ],
            meshes: [
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 0, NORMAL: 1, TEXCOORD_0: 2 },
                            indices: 3,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 4, NORMAL: 5, TEXCOORD_0: 6 },
                            indices: 7,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 8, NORMAL: 9, TEXCOORD_0: 10 },
                            indices: 11,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 12, NORMAL: 13, TEXCOORD_0: 14 },
                            indices: 15,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 16, NORMAL: 17, TEXCOORD_0: 18 },
                            indices: 19,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 20, NORMAL: 21, TEXCOORD_0: 22 },
                            indices: 23,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 24, NORMAL: 25, TEXCOORD_0: 26 },
                            indices: 27,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 28, NORMAL: 29, TEXCOORD_0: 30 },
                            indices: 31,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 32, NORMAL: 33, TEXCOORD_0: 34 },
                            indices: 35,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 36, NORMAL: 37, TEXCOORD_0: 38 },
                            indices: 39,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 40, NORMAL: 41, TEXCOORD_0: 42 },
                            indices: 43,
                            material: 0,
                        },
                    ],
                },
                {
                    primitives: [
                        {
                            mode: 4,
                            attributes: { POSITION: 44, NORMAL: 45, TEXCOORD_0: 46 },
                            indices: 47,
                            material: 0,
                        },
                    ],
                },
            ],
        };
    }

    /** 加载到场景并返回 Animator */
    async loadIntoScene(scene: THREE.Scene): Promise<PlayerAnimator> {
        const gltfJson = this.buildGltfJson();
        const loader = new GLTFLoader();

        const gltf = await loader.parseAsync(JSON.stringify(gltfJson), "");
        const player = gltf.scene;

        // 可选：缩放和位置调整
        player.scale.set(0.1, 0.1, 0.1);
        player.position.y = -0.8;

        scene.add(player);

        return new PlayerAnimator(player);
    }
}
