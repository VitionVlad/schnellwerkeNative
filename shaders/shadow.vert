#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
} ubo;

layout(binding = 1) uniform Model {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
} modelbuf;

void main() {
    gl_Position = ubo.view * modelbuf.model * vec4(pos.x, pos.y, pos.z, 1.0);
}