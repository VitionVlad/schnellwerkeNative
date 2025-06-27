#version 450

layout(location = 0) out vec4 outColor;

layout(location = 1) out vec4 outMaterial;

layout(location = 2) out vec4 outNormal;

layout(location = 3) out vec4 outPos;

layout(location = 0) in vec2 uv;

layout(location = 1) in vec4 pos;

layout(location = 2) in vec3 ftg;

layout(location = 3) in vec3 fctg;

layout(location = 4) in vec3 fnormal;

layout(binding = 0) uniform DefferedMatricesInput {
    mat4 defferedViews;
} dmi;

layout(binding = 1) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
    vec4 addinfo;
} mi;

layout(binding = 2) uniform sampler2DArray texSampler;

void main() {
    outColor = vec4(1.0, 1.0, 0.5, 1.0);
    outMaterial.r = 0.1;
    outMaterial.g = 0.1;
    outMaterial.b = 0.1;
    outNormal = vec4(fnormal, 1.0);
    outPos = pos;
}