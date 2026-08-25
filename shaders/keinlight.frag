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

void main() {
    vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
    outColor = vec4(SampleAlbedo(uv), 1.0);
    //outColor = voxelRaycast(rayOrigin, rayDir, outn);
}