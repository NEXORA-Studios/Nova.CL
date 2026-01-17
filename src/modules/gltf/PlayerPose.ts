export interface PoseNode {
    name: string;
    translation?: [number, number, number];
    rotation?: [number, number, number, number]; // x, y, z, w
    scale?: [number, number, number];
}

export class PlayerPose {
    constructor(public readonly name: string, public readonly nodes: PoseNode[] = []) {}

    /** 从两个完整的 nodes 数组中自动提取差异姿势 */
    static diff(idleNodes: any[], actionNodes: any[]): PlayerPose {
        const diff: PoseNode[] = [];

        actionNodes.forEach((actionNode, i) => {
            const idleNode = idleNodes[i];
            if (!actionNode.name || !idleNode) return;

            const t1 = actionNode.translation ?? idleNode.translation ?? [0, 0, 0];
            const r1 = actionNode.rotation ?? idleNode.rotation ?? [0, 0, 0, 1];
            const s1 = actionNode.scale ?? idleNode.scale ?? [1, 1, 1];

            const t0 = idleNode.translation ?? [0, 0, 0];
            const r0 = idleNode.rotation ?? [0, 0, 0, 1];
            const s0 = idleNode.scale ?? [1, 1, 1];

            const hasDiff = !arraysEqual(t1, t0) || !arraysEqual(r1, r0) || !arraysEqual(s1, s0);

            if (hasDiff) {
                diff.push({
                    name: actionNode.name,
                    translation: hasDiff && actionNode.translation ? t1 : undefined,
                    rotation: hasDiff && actionNode.rotation ? r1 : undefined,
                    scale: hasDiff && actionNode.scale ? s1 : undefined,
                });
            }
        });

        return new PlayerPose("custom", diff);
    }
}

function arraysEqual(a: number[], b: number[], eps = 1e-6): boolean {
    if (a.length !== b.length) return false;
    return a.every((v, i) => Math.abs(v - b[i]!) < eps);
}
