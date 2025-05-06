#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(location = 0) out vec2 fuv;

layout(location = 1) out vec4 fpos;

layout(location = 2) out vec4 fnormal;

layout(binding = 0) uniform DefferedMatricesInput {
    mat4 defferedView;
} dmi;

layout(binding = 1) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
} mi;

void main() {
    fuv = uv;
    fpos = vec4(pos.x, pos.y, pos.z, 1.0);
    fnormal = vec4(normal, 1.0);
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}