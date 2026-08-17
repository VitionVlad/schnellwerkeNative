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

layout(binding = 3)  uniform texture3D    texTexture;
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

vec4 voxelRaycast(inout vec3 rayOrigin, vec3 rayDir, out vec3 outn) {
    float voxelSize = mi.rtinfo.w;
    vec3  gridMin   = mi.rtinfo.xyz;
    vec3  gridSize  = mi.addinfo.yzw;
    vec3  gridMax   = gridMin + gridSize * voxelSize;

    outn = vec3(0.0, 1.0, 0.0);

    vec3  invDir = 1.0 / (rayDir + vec3(EPS));
    vec3  t0     = (gridMin - rayOrigin) * invDir;
    vec3  t1     = (gridMax - rayOrigin) * invDir;
    vec3  tMinV  = min(t0, t1);
    vec3  tMaxV2 = max(t0, t1);

    float tEnter = max(max(tMinV.x, tMinV.y), tMinV.z);
    float tExit  = min(min(tMaxV2.x, tMaxV2.y), tMaxV2.z);
    if (tEnter > tExit || tExit < 0.0) return vec4(0.0); // miss

    float t        = max(tEnter, 0.0);
    vec3  entryPos = rayOrigin + rayDir * (t + EPS);
    vec3  localPos = (entryPos - gridMin) / voxelSize;

    ivec3 voxel  = clamp(ivec3(floor(localPos)), ivec3(0), ivec3(gridSize) - 1);
    ivec3 vstep  = ivec3(sign(rayDir));
    vec3  tDelta = abs(voxelSize / rayDir);
    vec3  frac   = localPos - floor(localPos);

    vec3 tMaxV;
    tMaxV.x = (rayDir.x > 0.0 ? 1.0 - frac.x : frac.x) * tDelta.x;
    tMaxV.y = (rayDir.y > 0.0 ? 1.0 - frac.y : frac.y) * tDelta.y;
    tMaxV.z = (rayDir.z > 0.0 ? 1.0 - frac.z : frac.z) * tDelta.z;

    int  lastAxis = 0;
    bool sawEmpty = false;
    float tps     = 0.0;

    for (int i = 0; i < MAX_STEPS; i++) {
        if (any(lessThan(voxel, ivec3(0))) ||
            any(greaterThanEqual(voxel, ivec3(gridSize)))) {
            return vec4(0.0);
        }
        vec3 uvw = (vec3(voxel) + 0.5) / gridSize;
        vec4 cl  = texture(sampler3D(texTexture, attachmentSampler), uvw);
        if (cl.a > 0.5) {
            if (sawEmpty) {
                vec3 n;
                if (lastAxis == 0) n = vec3(-float(vstep.x), 0.0, 0.0);
                else if (lastAxis == 1) n = vec3(0.0, -float(vstep.y), 0.0);
                else n = vec3(0.0, 0.0, -float(vstep.z));
                outn      = n;
                rayOrigin = rayOrigin + rayDir * tps;
                return vec4(cl.rgb, 1.0);
            }
        } else {
            sawEmpty = true;
        }
        if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
            tps      = tMaxV.x; tMaxV.x += tDelta.x;
            voxel.x += vstep.x; lastAxis  = 0;
        } else if (tMaxV.y < tMaxV.z) {
            tps      = tMaxV.y; tMaxV.y += tDelta.y;
            voxel.y += vstep.y; lastAxis  = 1;
        } else {
            tps      = tMaxV.z; tMaxV.z += tDelta.z;
            voxel.z += vstep.z; lastAxis  = 2;
        }
    }

    return vec4(0.0);
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
    //outColor = SVORaycast(rayOrigin, rayDir, gridMin, gridMax, 5, outp, outn);
    outColor = voxelRaycast(rayOrigin, rayDir, outn);
}