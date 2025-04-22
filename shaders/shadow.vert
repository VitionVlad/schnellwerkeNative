#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(location = 0) out vec2 fuv;

layout(binding = 0) uniform UniformBufferObject {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
} ubo;

void main() {
    fuv = uv;
    gl_Position = vec4(pos.x, pos.y, ubo.lightinfo.y, 1.0);
}