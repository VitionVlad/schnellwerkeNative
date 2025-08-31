#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
} ubo;

layout(binding = 1) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 t;
    mat4 r;
    mat4 s;
    vec4 addinfo;
} mi;

void main() {
    gl_Position = ubo.view * mi.t * mi.r * mi.s * vec4(pos.x, pos.y, pos.z, 1.0);
}