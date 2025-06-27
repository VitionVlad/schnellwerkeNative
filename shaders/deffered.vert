#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(location = 0) out vec2 fuv;

layout(location = 1) out vec4 fpos;

layout(location = 2) out vec3 ftg;

layout(location = 3) out vec3 fctg;

layout(location = 4) out vec3 fnormal;

layout(binding = 0) uniform DefferedMatricesInput {
    mat4 defferedView;
} dmi;

layout(binding = 1) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
    vec4 addinfo;
} mi;

void main() {
    fuv = uv;
    fpos = mi.model * vec4(pos.x, pos.y, pos.z, 1.0);
    ftg = tg;
    fctg = ctg;
    fnormal = normal;
    gl_Position = dmi.defferedView * mi.model * vec4(pos.x, pos.y, pos.z, 1.0);
}