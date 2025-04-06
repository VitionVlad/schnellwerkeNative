#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 uv;

layout(binding = 0) uniform UniformBufferObject {
    vec4 resolutions;
    mat4 model;
    vec4 adddata;
} ubo;

void main() {
    outColor = vec4(uv, sin(ubo.resolutions.w), 1.0);
}