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

const int SVO_LEAF_MARKER       = 0x00;
const int SVO_BRANCH_MARKER     = 0xFF;
const int SVO_NODE_SIZE         = 5;
const int SVO_LEAF_SIZE         = 5;
const int SVO_BRANCH_HEADER_SIZE = 5;

const int SVO_MAX_DEPTH = 20;

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
        default: octMin = boxMin; octMax = mid; break;
    }
}

uint GetNodeSize(uint nodePtr) {
    uint marker = SVO_Byte(nodePtr);
    
    if (marker == SVO_LEAF_MARKER) {
        return 5u;
    } 
    
    if (marker == SVO_BRANCH_MARKER) {
        return SVO_U32(nodePtr+1);
    }
    
    return 5u;
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

vec3 SVO_BoxNormal(vec3 ro, vec3 rd, vec3 bMin, vec3 bMax, float t) {
    vec3 p = ro + rd * t;
    vec3 c = (bMin + bMax) * 0.5;
    vec3 h = (bMax - bMin) * 0.5;
    vec3 local = (p - c) / h;
    vec3 a = abs(local);
    if (a.x > a.y && a.x > a.z) return vec3(sign(local.x), 0.0, 0.0);
    if (a.y > a.z)              return vec3(0.0, sign(local.y), 0.0);
    return vec3(0.0, 0.0, sign(local.z));
}

uint SVO_GetChildPtr(uint nodePtr, int childIdx) {
    uint ptr = nodePtr + uint(SVO_BRANCH_HEADER_SIZE);
    for (int i = 0; i < childIdx; i++) {
        ptr += GetNodeSize(ptr);
    }
    return ptr;
}

vec4 SVORaycast(vec3 ro, vec3 rd, vec3 gridMin, vec3 gridMax, int maxDepth, out vec3 hitPos, out vec3 hitNormal) {
    hitPos = vec3(0.0);
    hitNormal = vec3(0.0);

    float tEnter, tExit;
    if (!SVO_RayAABB(ro, rd, gridMin, gridMax, tEnter, tExit)) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    uint stkNodePtr[SVO_MAX_DEPTH];
    vec3 stkBoxMin[SVO_MAX_DEPTH];
    vec3 stkBoxMax[SVO_MAX_DEPTH];
    int  stkOrder[SVO_MAX_DEPTH * 8];
    int  stkCount[SVO_MAX_DEPTH];
    int  stkCursor[SVO_MAX_DEPTH];

    int cappedMaxDepth = min(maxDepth, SVO_MAX_DEPTH);

    int depth = 0;
    uint nodePtr = 0u;
    vec3 boxMin = gridMin;
    vec3 boxMax = gridMax;

    for (int iter = 0; iter < SVO_MAX_DEPTH * 64; iter++) {
        uint marker = SVO_Byte(nodePtr);
        bool descended = false;

        if (marker == SVO_BRANCH_MARKER && depth < cappedMaxDepth) {
            vec3 mid = (boxMin + boxMax) * 0.5;
            int   idxBuf[8];
            float tBuf[8];
            int   n = 0;

            for (int i = 0; i < 8; i++) {
                vec3 cMin, cMax;
                GetOctantBounds(i, boxMin, boxMax, mid, cMin, cMax);
                float cEnter, cExit;
                if (SVO_RayAABB(ro, rd, cMin, cMax, cEnter, cExit) && cExit >= 0.0) {
                    idxBuf[n] = i;
                    tBuf[n]   = max(cEnter, 0.0);
                    n++;
                }
            }

            // insertion sort, nearest-first (n <= 8)
            for (int i = 1; i < n; i++) {
                float kt = tBuf[i]; int ki = idxBuf[i];
                int j = i - 1;
                while (j >= 0 && tBuf[j] > kt) {
                    tBuf[j+1] = tBuf[j]; idxBuf[j+1] = idxBuf[j]; j--;
                }
                tBuf[j+1] = kt; idxBuf[j+1] = ki;
            }

            if (n > 0) {
                stkNodePtr[depth] = nodePtr;
                stkBoxMin[depth]  = boxMin;
                stkBoxMax[depth]  = boxMax;
                stkCount[depth]   = n;
                stkCursor[depth]  = 0;
                for (int i = 0; i < n; i++) stkOrder[depth*8 + i] = idxBuf[i];

                int oct = idxBuf[0];
                nodePtr = SVO_GetChildPtr(nodePtr, oct);
                GetOctantBounds(oct, boxMin, boxMax, mid, boxMin, boxMax);
                depth++;
                descended = true;
            }
        } else if (marker == SVO_LEAF_MARKER) {
            uint a = SVO_Byte(nodePtr + 4u);
            if (a != 0u) {
                float lEnter, lExit;
                if (SVO_RayAABB(ro, rd, boxMin, boxMax, lEnter, lExit)) {
                    float tHit = max(lEnter, 0.0);
                    hitPos    = ro + rd * tHit;
                    hitNormal = SVO_BoxNormal(ro, rd, boxMin, boxMax, tHit);
                    return vec4(
                        float(SVO_Byte(nodePtr + 1u)) / 255.0,
                        float(SVO_Byte(nodePtr + 2u)) / 255.0,
                        float(SVO_Byte(nodePtr + 3u)) / 255.0,
                        float(a) / 255.0);
                }
            }
        }

        if (descended) continue;

        bool exhausted = false;
        while (true) {
            if (depth == 0) { exhausted = true; break; }
            depth--;
            stkCursor[depth]++;
            if (stkCursor[depth] < stkCount[depth]) {
                int oct = stkOrder[depth*8 + stkCursor[depth]];
                nodePtr = SVO_GetChildPtr(stkNodePtr[depth], oct);
                vec3 pMin = stkBoxMin[depth];
                vec3 pMax = stkBoxMax[depth];
                vec3 pMid = (pMin + pMax) * 0.5;
                GetOctantBounds(oct, pMin, pMax, pMid, boxMin, boxMax);
                depth++;
                break;
            }
        }
        if (exhausted) break;
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