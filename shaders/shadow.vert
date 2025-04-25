#version 450

layout(location = 0) in vec3 pos;

layout(location = 1) in vec2 uv;

layout(location = 2) in vec3 normal;

layout(location = 3) in vec3 tg;

layout(location = 4) in vec3 ctg;

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
} ubo;

void main() {
    gl_Position = vec4(pos.x, pos.y, ubo.view[0][0], 1.0);
}