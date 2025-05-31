#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(location = 0) out vec2 fuv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
    vec4 addinfo;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedViews[10];
    vec4 campos[10];
    vec4 camrot[10];
} dmi;

void main() {
    fuv = uv;
    vec4 fpos = mi.model * vec4(pos, 1.0);
    gl_Position = vec4(fpos.x/mi.resolutions.x, fpos.y/mi.resolutions.y, fpos.z, fpos.w);
}