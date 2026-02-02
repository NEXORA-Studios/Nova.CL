/**
 * 披风的预定义动画帧（对应 CapeGltfBuilder 中的 NodeMap）
 * 每帧只影响披风根节点的 rotation（其他部分保持不变）
 */
export interface CapeFrame {
    rotation: [number, number, number, number]; // Quaternion [x, y, z, w]
}

export interface PoseNode {
    name: string;
    translation?: [number, number, number];
    rotation?: [number, number, number, number]; // x, y, z, w
    scale?: [number, number, number];
}

export interface GltfBuffers {
    byteLength: number;
    uri: string;
}

export interface GltfAccessor {
    bufferView: number;
    componentType: number;
    count: number;
    max: number[];
    min: number[];
    type: string;
}

export interface GltfNode {
    name: string;
    rotation?: number[];
    translation?: number[];
    children?: number[];
    mesh?: number;
}
