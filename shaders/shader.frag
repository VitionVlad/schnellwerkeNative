#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 uv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedViews[10];
    vec4 lightpos[10];
    vec4 lightcol[10];
} dmi;

layout(binding = 3) uniform sampler2DArray texSampler;

layout(binding = 4) uniform sampler2DArray defferedSampler;

layout(binding = 5) uniform sampler2DArray defferedDepthSampler;

layout(binding = 6) uniform sampler2DArray shadowSampler;

void main() {
    if(uv.x > 0.5){
        outColor = vec4(texture(defferedSampler, vec3(uv, 1)).rgb, 1.0);
    }else{
        outColor = vec4(texture(defferedSampler, vec3(uv, 0)).rgb, 1.0);
    }
}