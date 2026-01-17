import * as THREE from "three";
import { PlayerPose } from ".";

export class PlayerAnimator {
    private parts = new Map<string, THREE.Object3D>();
    private root: THREE.Group;

    // 新增：保存加载时的原始 idle 状态
    private idleStates = new Map<string, { pos: THREE.Vector3; quat: THREE.Quaternion }>();

    private isAnimating = false;
    private progress = 0;
    private duration = 0;
    private startTime = 0;
    private startStates = new Map<string, { pos: THREE.Vector3; quat: THREE.Quaternion }>();
    private targetPose: PlayerPose | null = null;

    constructor(root: THREE.Group) {
        this.root = root;
        this.collectParts();
        this.saveIdleStates(); // 关键：保存原始状态
    }

    private collectParts() {
        const nameMap: Record<string, string> = {
            Head: "Head",
            Body: "Body",
            Right_Arm: "Right Arm",
            Left_Arm: "Left Arm",
            Right_Leg: "Right Leg",
            Left_Leg: "Left Leg",
            Waist: "Waist",
        };

        this.root.traverse((obj) => {
            if (obj.type === "Object3D" && obj.name && nameMap[obj.name]) {
                this.parts.set(nameMap[obj.name]!, obj);
            }
        });
    }

    /** 保存模型加载时的原始 idle 状态（作为回正目标） */
    private saveIdleStates() {
        this.parts.forEach((part, name) => {
            this.idleStates.set(name, {
                pos: part.position.clone(),
                quat: part.quaternion.clone(),
            });
        });
    }

    transitionTo(pose: PlayerPose, durationMs: number = 600) {
        this.targetPose = pose;

        // 始终记录当前状态作为起始
        this.startStates.clear();
        this.parts.forEach((part, name) => {
            this.startStates.set(name, {
                pos: part.position.clone(),
                quat: part.quaternion.clone(),
            });
        });

        this.isAnimating = true;
        this.progress = 0;
        this.duration = durationMs;
        this.startTime = performance.now();
    }

    update() {
        if (!this.isAnimating) return;

        const elapsed = performance.now() - this.startTime;
        this.progress = Math.min(elapsed / this.duration, 1);

        const t = this.progress < 0.5 ? 2 * this.progress * this.progress : 1 - Math.pow(-2 * this.progress + 2, 2) / 2;

        if (this.targetPose!.nodes.length === 0) {
            // 空姿势：回正到保存的 idle 状态
            this.parts.forEach((part, name) => {
                const start = this.startStates.get(name)!;
                const idle = this.idleStates.get(name)!;

                part.position.lerpVectors(start.pos, idle.pos, t);
                part.quaternion.slerpQuaternions(start.quat, idle.quat, t);
            });
        } else {
            // 有动作姿势：只对有定义的节点插值
            this.targetPose!.nodes.forEach((node) => {
                const part = this.parts.get(node.name);
                if (!part) return;

                const start = this.startStates.get(node.name)!;

                if (node.translation) {
                    const targetPos = new THREE.Vector3(...node.translation);
                    part.position.lerpVectors(start.pos, targetPos, t);
                }

                if (node.rotation) {
                    const targetQuat = new THREE.Quaternion(...node.rotation);
                    part.quaternion.slerpQuaternions(start.quat, targetQuat, t);
                }
            });
        }

        if (this.progress >= 1) {
            this.isAnimating = false;
        }
    }

    get isPlaying(): boolean {
        return this.isAnimating;
    }
}
