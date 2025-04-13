#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 uv;

layout(binding = 0) uniform UniformBufferObject {
    vec4 resolutions;
    mat4 model;
    vec4 adddata;
} ubo;

layout(binding = 1) uniform sampler2DArray texSampler;

void main() {
    outColor = vec4(texture(texSampler, vec3(uv, 0)).rgb, 1.0);
}