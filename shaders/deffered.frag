#version 450

layout(location = 0) out vec4 outColor;

layout(location = 1) out vec4 outMaterial;

layout(location = 2) out vec4 outNormal;

layout(location = 3) out vec4 outPos;

layout(location = 0) in vec2 uv;

layout(location = 1) in vec4 pos;

layout(location = 2) in vec4 normal;

layout(binding = 0) uniform DefferedMatricesInput {
    mat4 defferedViews;
} dmi;

layout(binding = 1) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
} mi;

layout(binding = 2) uniform sampler2DArray texSampler;

void main() {
    outColor = vec4(texture(texSampler, vec3(uv, 0)).rgb, 1.0);
    outMaterial = vec4(texture(texSampler, vec3(uv, 1)).rgb, 1.0);
    outNormal = normal;
    outPos = pos;
}