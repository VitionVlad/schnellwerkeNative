#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 fuv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 t;
    mat4 r;
    mat4 s;
    vec4 addinfo;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedMVP[10];
    mat4 defferedMVPInverse[10];
    vec4 deffpos[10];
    vec4 deffrot[10];
} dmi;

layout(binding = 3) uniform texture2DArray texTexture;

layout(binding = 4) uniform texture2D defferedTexture;

layout(binding = 5) uniform texture2D defferedDepthTexture;

layout(binding = 6) uniform texture2DArray shadowTexture;

layout(binding = 7) uniform sampler imageSampler;

layout(binding = 8) uniform sampler attachmentSampler;

const float SIGMA_COLOR = 0.15;

const float KERNEL[25] = float[25](
    1.0/256.0,  4.0/256.0,  6.0/256.0,  4.0/256.0, 1.0/256.0,
    4.0/256.0, 16.0/256.0, 24.0/256.0, 16.0/256.0, 4.0/256.0,
    6.0/256.0, 24.0/256.0, 36.0/256.0, 24.0/256.0, 6.0/256.0,
    4.0/256.0, 16.0/256.0, 24.0/256.0, 16.0/256.0, 4.0/256.0,
    1.0/256.0,  4.0/256.0,  6.0/256.0,  4.0/256.0, 1.0/256.0
);

float Luminance(vec3 c) {
    return dot(c, vec3(0.2126, 0.7152, 0.0722));
}

vec3 SampleRT(vec2 uv) {
    return texture(sampler2D(defferedTexture, attachmentSampler), uv).rgb;
}

void main() {
    vec2 uv = fuv;

    //vec2 texelSize = 1.0 / mi.resolutions.xy;
    //int   passIndex = int(mi.addinfo.y);
    //float stepSize  = float(1 << passIndex);
//
    //vec3  centerColor = SampleRT(uv);
    //float centerLum   = Luminance(centerColor);
//
    //float sigmaLum = max(mi.addinfo.x, 0.01);
//
    //vec3  colorAccum  = vec3(0.0);
    //float weightAccum = 0.0;
//
    //for (int row = 0; row < 5; row++) {
    //    for (int col = 0; col < 5; col++) {
    //        vec2 offset   = vec2(float(col - 2), float(row - 2)) * texelSize * stepSize;
    //        vec2 sampleUV = clamp(uv + offset, vec2(0.0), vec2(1.0));
//
    //        vec3  sampleColor = SampleRT(sampleUV);
    //        float sampleLum   = Luminance(sampleColor);
//
    //        float wKernel = KERNEL[row * 5 + col];
//
    //        float lumDiff = centerLum - sampleLum;
    //        float wLum    = exp(-(lumDiff * lumDiff) / (2.0 * sigmaLum * sigmaLum + 0.0001));
//
    //        vec3  colorDiff = centerColor - sampleColor;
    //        float colorDist = dot(colorDiff, colorDiff);
    //        float wColor    = exp(-colorDist / (2.0 * SIGMA_COLOR * SIGMA_COLOR + 0.0001));
//
    //        float w = wKernel * wLum * wColor;
//
    //        colorAccum  += sampleColor * w;
    //        weightAccum += w;
    //    }
    //}
//
    //outColor = vec4(colorAccum / max(weightAccum, 0.0001), 1.0);

    outColor = vec4(SampleRT(uv), 1.0);
}