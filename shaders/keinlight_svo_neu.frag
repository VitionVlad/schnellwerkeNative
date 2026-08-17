#version 450

layout(location = 0) out vec4 outColor;
layout(location = 0) in  vec2 fuv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 t;
    mat4 r;
    mat4 s;
    vec4 addinfo;
    vec4 rtinfo;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedMVP[10];
    mat4 defferedMVPInverse[10];
    vec4 deffpos[10];
    vec4 deffrot[10];
} dmi;

layout(binding = 3)  uniform utexture2D    texTexture;
layout(binding = 4)  uniform texture2DArray defferedTexture;
layout(binding = 5)  uniform texture2DArray defferedDepthTexture;
layout(binding = 6)  uniform texture2DArray shadowTexture;
layout(binding = 7)  uniform sampler        imageSampler;
layout(binding = 8)  uniform sampler        attachmentSampler;
layout(binding = 9)  uniform texture2D      noiseTexture;
layout(binding = 10) uniform sampler        noiseSampler;

const float PI           = 3.14159265359;
const float GOLDEN_RATIO = 0.61803398875;
const int   MAX_STEPS    = 128;
const float EPS          = 1e-5;
const float SS_THICKNESS = 0.3;
const vec3  SKY_COLOR    = vec3(0.02, 0.02, 0.05);
//const vec3  SKY_COLOR    = vec3(0.67, 0.84, 0.89);
const float FIREFLY_CAP  = 10.0;
const bool  DEBUG_OCTANTS = false;  // Set to true to visualize octants by index                  

vec3 SampleAlbedo(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
}

vec3 SampleRMA(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
}

vec3 SampleNormal(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
}

float SampleDepth(vec2 uv) {
    return texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
}

uint SVO_Byte(uint idx) {
    return texelFetch(usampler2D(texTexture, attachmentSampler), ivec2(idx, 0), 0).r;
}

uint SVO_U32(uint idx) {
    uint b0 = SVO_Byte(idx + 0);
    uint b1 = SVO_Byte(idx + 1);
    uint b2 = SVO_Byte(idx + 2);
    uint b3 = SVO_Byte(idx + 3);
    return b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
}

const uint SVO_LEAF_MARKER = 0u;
const uint SVO_BRANCH_MARKER = 100u;
const uint SVO_LEAF_SIZE = 5u;
const uint SVO_BRANCH_HEADER_SIZE= 2u;

const int  SVO_MAX_DEPTH = 20;
const uint SVO_LEVELS = 8u;

void GetOctantBounds(int octantIdx, vec3 boxMin, vec3 boxMax, vec3 mid, out vec3 octMin, out vec3 octMax) {
    switch (octantIdx) {
        case 0: octMin = boxMin; octMax = mid; break;
        case 1: octMin = vec3(mid.x, boxMin.y, boxMin.z); octMax = vec3(boxMax.x, mid.y, mid.z); break;
        case 2: octMin = vec3(boxMin.x, mid.y, boxMin.z); octMax = vec3(mid.x, boxMax.y, mid.z); break;
        case 3: octMin = vec3(mid.x, mid.y, boxMin.z); octMax = vec3(boxMax.x, boxMax.y, mid.z); break;
        case 4: octMin = vec3(boxMin.x, boxMin.y, mid.z); octMax = vec3(mid.x, mid.y, boxMax.z); break;
        case 5: octMin = vec3(mid.x, boxMin.y, mid.z); octMax = vec3(boxMax.x, mid.y, boxMax.z); break;
        case 6: octMin = vec3(boxMin.x, mid.y, mid.z); octMax = vec3(mid.x, boxMax.y, boxMax.z); break;
        case 7: octMin = mid; octMax = boxMax; break;
        default: octMin = boxMin; octMax = boxMax; break;
    }
}

//uint GetChildOffset(uint branchPtr, int childIdx) {
//    return branchPtr + uint(SVO_BRANCH_HEADER_SIZE) + uint(childIdx) * uint(SVO_NODE_SIZE);
//}

bool SVO_RayAABB(vec3 ro, vec3 rd, vec3 bMin, vec3 bMax, out float tEnter, out float tExit) {
    vec3 rdInv = vec3(
        abs(rd.x) < 1e-9 ? 1e38 : 1.0 / rd.x,
        abs(rd.y) < 1e-9 ? 1e38 : 1.0 / rd.y,
        abs(rd.z) < 1e-9 ? 1e38 : 1.0 / rd.z
    );
    
    vec3 t0   = (bMin - ro) * rdInv;
    vec3 t1   = (bMax - ro) * rdInv;
    vec3 tMin = min(t0, t1);
    vec3 tMax = max(t0, t1);
    tEnter    = max(max(tMin.x, tMin.y), tMin.z);
    tExit     = min(min(tMax.x, tMax.y), tMax.z);
    return tEnter <= tExit && tExit >= 0.0;
}

vec4 SVO_LeafColor(uint leafPtr) {
    return vec4(
        float(SVO_Byte(leafPtr + 1u)) / 255.0,
        float(SVO_Byte(leafPtr + 2u)) / 255.0,
        float(SVO_Byte(leafPtr + 3u)) / 255.0,
        float(SVO_Byte(leafPtr + 4u)) / 255.0
    );
}

vec4 SVORaycast(vec3 ro, vec3 rd, vec3 gridMin, vec3 gridMax, int maxDepth, out vec3 hitPos, out vec3 hitNormal) {
    hitPos = vec3(0.0);
    hitNormal = vec3(0.0);

    float rootTEnter, rootTExit;
    if (!SVO_RayAABB(ro, rd, gridMin, gridMax, rootTEnter, rootTExit)) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    // Layer table: absolute byte offset where each depth's node data begins.
    uint stkNodePtr[SVO_MAX_DEPTH + 1];
    stkNodePtr[0] = uint(maxDepth) * 4u;
    {
        uint headerPtr = 0u;
        for (int i = 0; i < maxDepth; i++) {
            stkNodePtr[i + 1] = SVO_U32(headerPtr);
            headerPtr += 4u;
        }
    }

    uint stkBranchPtr[SVO_MAX_DEPTH];
    uint stkChildIdx[SVO_MAX_DEPTH];
    uint stkChildCount[SVO_MAX_DEPTH];
    vec3 stkBoxMin[SVO_MAX_DEPTH];
    vec3 stkBoxMax[SVO_MAX_DEPTH];

    int current_layer = 0;
    uint nodeptr = stkNodePtr[0];
    stkBoxMin[0] = gridMin;
    stkBoxMax[0] = gridMax;

    for (int iter = 0; iter < SVO_MAX_DEPTH * 64; iter++) {
        uint marker = SVO_Byte(nodeptr);
        vec3 pBoxMin = stkBoxMin[current_layer];
        vec3 pBoxMax = stkBoxMax[current_layer];

        vec3 ownMin, ownMax;
        if (current_layer == 0) {
            ownMin = pBoxMin;
            ownMax = pBoxMax;
        } else {
            int ownOctant = int(marker < SVO_BRANCH_MARKER ? marker : marker - SVO_BRANCH_MARKER);
            GetOctantBounds(ownOctant, pBoxMin, pBoxMax, (pBoxMin + pBoxMax) * 0.5, ownMin, ownMax);
        }

        float te, tm;
        bool hitsBox;
        if (current_layer == 0) {
            hitsBox = true;
            te = rootTEnter;
            tm = rootTExit;
        } else {
            hitsBox = SVO_RayAABB(ro, rd, ownMin, ownMax, te, tm);
        }

        bool descended = false;

        if (marker < SVO_BRANCH_MARKER) {
            if (hitsBox) {
                vec4 lfc = SVO_LeafColor(nodeptr);
                if (lfc.a != 0.0) {
                    hitPos = ro + rd * te;
                    return lfc;
                }
            }
        } else {
            uint childCount = SVO_Byte(nodeptr + 1u);
            if (hitsBox && childCount > 0u) {
                stkBranchPtr[current_layer] = nodeptr;
                stkChildCount[current_layer] = childCount;
                stkChildIdx[current_layer] = 0u;
                stkBoxMin[current_layer + 1] = ownMin;
                stkBoxMax[current_layer + 1] = ownMax;

                uint childRel = SVO_U32(nodeptr + 2u);
                nodeptr = stkNodePtr[current_layer + 1] + childRel;
                current_layer++;
                descended = true;
            }
        }
        if (descended) {
            continue;
        }
        current_layer--;
        while (current_layer >= 0) {
            stkChildIdx[current_layer]++;
            if (stkChildIdx[current_layer] < stkChildCount[current_layer]) {
                uint branchPtr = stkBranchPtr[current_layer];
                uint childRel = SVO_U32(branchPtr + 2u + stkChildIdx[current_layer] * 4u);
                nodeptr = stkNodePtr[current_layer + 1] + childRel;
                current_layer++;
                break;
            }
            current_layer--;
        }
        if (current_layer < 0) {
            break;
        }
    }

    return vec4(SKY_COLOR, 1.0);
}

vec3 get_raydir(vec2 uv, float fov, inout vec3 camForward, inout vec3 camRight, inout vec3 camUp) {
    float pitch = dmi.deffrot[0].x;
    float yaw   = dmi.deffrot[0].y;

    camForward = normalize(vec3(
        cos(pitch) * sin(yaw),
       -sin(pitch),
       -cos(pitch) * cos(yaw)
    ));

    vec3 worldUp = vec3(0.0, 1.0, 0.0);
    camRight = normalize(cross(camForward, worldUp));
    camUp = normalize(cross(camRight, camForward));

    vec2  screen = uv * 2.0 - 1.0;
    float aspect = mi.resolutions.x / mi.resolutions.y;
    float tansHalfFov = tan(radians(fov) * 0.5);

    return normalize(camForward + camRight * screen.x * aspect * tansHalfFov + camUp * screen.y * tansHalfFov);
}

void main() {
    vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
    //outColor = vec4(SampleAlbedo(uv), 1.0);
    vec3 camForward, camRight, camUp;
    float fovDeg = dmi.deffrot[0].w;
    vec3 rayOrigin = dmi.deffpos[0].xyz;
    vec3 rayDir = get_raydir(uv, fovDeg, camForward, camRight, camUp);
    vec3 outn, outp;
    float voxelSize = mi.rtinfo.w;
    vec3 gridMin = mi.rtinfo.xyz;
    vec3 gridSize = mi.addinfo.yzw;
    vec3 gridMax = gridMin + gridSize * voxelSize;
    outColor = SVORaycast(rayOrigin, rayDir, gridMin, gridMax, 5, outp, outn);
    //outColor = voxelRaycast(rayOrigin, rayDir, outn);
}