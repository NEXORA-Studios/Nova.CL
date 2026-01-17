import * as THREE from "three";

/**
 * 披风的预定义动画帧（对应 CapeGltfBuilder 中的 NodeMap）
 * 每帧只影响披风根节点的 rotation（其他部分保持不变）
 */
interface CapeFrame {
    rotation: [number, number, number, number]; // Quaternion [x, y, z, w]
}

const CAPE_FRAMES: CapeFrame[] = [
    // step 0: 静止下垂
    { rotation: [0, -1, 0, 6.123233995736766e-17] },
    // step 1
    { rotation: [7.248032245908688e-16, 0.9986295347545738, 0.052335956242943966, 6.795037343273465e-17] },
    // step 2
    { rotation: [7.20253659191127e-16, 0.9945218953682733, 0.10452846326765346, 1.0579051965446356e-16] },
    // step 3
    { rotation: [7.149536196463327e-16, 0.9890158633619168, 0.14780941112961077, 1.3710685388257098e-16] },
    // step 4: 最大摆动
    { rotation: [7.075521391385794e-16, 0.9807852804032304, 0.19509032201612833, 1.7125109642682566e-16] },
];

export class CapeAnimator {
    private root: THREE.Group;
    private capeRoot: THREE.Object3D | null = null; // 披风的根节点（有 rotation 的那个）

    // 保存初始静止状态（step 0），用于回正或循环动画起点
    private idleQuaternion: THREE.Quaternion;

    private isAnimating = false;
    private progress = 0;
    private duration = 0;
    private startTime = 0;
    private startQuaternion: THREE.Quaternion = new THREE.Quaternion();
    private targetQuaternion: THREE.Quaternion = new THREE.Quaternion();

    constructor(root: THREE.Group) {
        this.root = root;
        this.findCapeRoot();
        if (!this.capeRoot) {
            throw new Error("CapeAnimator: Could not find cape root node in the loaded model");
        }

        // 保存初始 idle 状态（加载时默认是 step 0）
        this.idleQuaternion = this.capeRoot.quaternion.clone();
    }

    private findCapeRoot() {
        // 在 CapeGltfBuilder 生成的模型中，nodes[1] 是带有 rotation 的根节点
        // 它通常是 scene 的直接子节点，且有 children
        this.root.traverse((obj) => {
            if (obj.type === "Object3D" && obj.name === "cape" && obj.children.length > 0) {
                this.capeRoot = obj;
            }
        });

        // 备选方案：找第一个有 rotation 且不是 leaf 的节点
        if (!this.capeRoot) {
            this.root.traverse((obj) => {
                if (obj.type === "Object3D" && obj.children.length > 0) {
                    this.capeRoot = obj;
                    return;
                }
            });
        }
    }

    /**
     * 过渡到指定动画帧
     * @param step 0~4，对应预定义的披风摆动程度
     * @param durationMs 过渡时间（毫秒），默认 300ms
     */
    transitionTo(step: number, durationMs: number = 300) {
        if (step < 0 || step >= CAPE_FRAMES.length) {
            console.tWarn({ category: "Cape Animator", message: `step ${step} out of range, clamping to 0-${CAPE_FRAMES.length - 1}` });
            step = Math.max(0, Math.min(step, CAPE_FRAMES.length - 1));
        }

        const frame = CAPE_FRAMES[step];
        this.targetQuaternion.set(...frame!.rotation);

        this.startQuaternion.copy(this.capeRoot!.quaternion);

        this.isAnimating = true;
        this.progress = 0;
        this.duration = durationMs;
        this.startTime = performance.now();
    }

    /**
     * 播放一个完整的摆动循环（0 → 4 → 0），适合行走/奔跑时使用
     * @param cycleDurationMs 完整一次往返的时间
     */
    playSwingCycle(cycleDurationMs: number = 800) {
        // 可以扩展为更复杂的循环状态机，这里提供简单实现示例
        // 实际使用中建议配合外部计时器或玩家移动状态调用 transitionTo
        this.transitionTo(4, cycleDurationMs / 2);
        // 实际项目中通常会在外部 update 中根据速度动态驱动
    }

    /**
     * 回正到静止状态（下垂）
     */
    resetToIdle(durationMs: number = 400) {
        this.targetQuaternion.copy(this.idleQuaternion);
        this.startQuaternion.copy(this.capeRoot!.quaternion);

        this.isAnimating = true;
        this.progress = 0;
        this.duration = durationMs;
        this.startTime = performance.now();
    }

    update() {
        if (!this.isAnimating || !this.capeRoot) return;

        const elapsed = performance.now() - this.startTime;
        this.progress = Math.min(elapsed / this.duration, 1);

        // 使用 ease-in-out 插值
        const t = this.progress < 0.5 ? 2 * this.progress * this.progress : 1 - Math.pow(-2 * this.progress + 2, 2) / 2;

        this.capeRoot.quaternion.slerpQuaternions(this.startQuaternion, this.targetQuaternion, t);

        if (this.progress >= 1) {
            this.isAnimating = false;
        }
    }

    get isPlaying(): boolean {
        return this.isAnimating;
    }

    /** 获取当前披风根节点，便于外部挂靠到玩家身体 */
    get capeObject(): THREE.Object3D | null {
        return this.capeRoot;
    }
}
