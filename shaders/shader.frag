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
    vec4 rtinfo;
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

layout(binding = 4) uniform texture2DArray lightingTexture;

layout(binding = 5) uniform texture2DArray lightingDepthTexture;

layout(binding = 6) uniform texture2DArray defferedTexture;

layout(binding = 7) uniform texture2DArray defferedDepthTexture;

layout(binding = 8) uniform texture2DArray shadowTexture;

layout(binding = 9) uniform sampler imageSampler;

layout(binding = 10) uniform sampler attachmentSampler;

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
    return texture(sampler2DArray(lightingTexture, attachmentSampler), vec3(uv, mi.resolutions.a)).rgb;
}

vec3 SampleRTl(vec3 uv) {
    return texture(sampler2DArray(lightingTexture, attachmentSampler), uv).rgb;
}

vec3 SampleRTavg(vec2 uv) {
    return (SampleRTl(vec3(uv, 0)) + SampleRTl(vec3(uv, 1)))/2.0;
}

vec3 SampleRTsum(vec2 uv) {
    return SampleRTl(vec3(uv, 0)) + SampleRTl(vec3(uv, 1));
}

vec3 SampleNormal(vec2 uv) {
    return normalize(
        texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb
    );
}

float SampleDepth(vec2 uv) {
    return texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
}

float EstimateVariance(vec2 uv, vec2 texelSize) {
    float sumL  = 0.0;
    float sumL2 = 0.0;

    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            float lum = Luminance(SampleRT(uv + vec2(dx, dy) * texelSize));
            sumL  += lum;
            sumL2 += lum * lum;
        }
    }

    float mean     = sumL / 9.0;
    float variance = max((sumL2 / 9.0) - (mean * mean), 0.0);
    return variance;
}

float NormalWeight(vec3 nA, vec3 nB, float sigmaNormal) {
    float d = max(dot(nA, nB), 0.0);
    return pow(d, 1.0 / max(sigmaNormal, 0.001));
}

float DepthWeight(float dA, float dB) {
    const float SIGMA_DEPTH = 0.02;
    float diff = abs(dA - dB) / (abs(dA) + 0.001);
    return exp(-diff / (SIGMA_DEPTH + 0.001));
}

float LuminanceWeight(float lumA, float lumB, float adaptiveSigma) {
    float diff = abs(lumA - lumB);
    return exp(-(diff * diff) / (2.0 * adaptiveSigma * adaptiveSigma + 0.0001));
}

float ColorWeight(vec3 cA, vec3 cB) {
    vec3  diff = cA - cB;
    float dist = dot(diff, diff);
    return exp(-dist / (2.0 * SIGMA_COLOR * SIGMA_COLOR + 0.0001));
}

vec3 keffect(vec2 uv){
  vec2 offset = vec2(1.0 / mi.resolutions.x, 1.0 / mi.resolutions.y);
  vec2 offsets[9] = vec2[](
    vec2(-offset.x,  offset.y),
    vec2( 0.0f,    offset.y),
    vec2( offset.x,  offset.y),
    vec2(-offset.x,  0.0f),  
    vec2( 0.0f,    0.0f),  
    vec2( offset.x,  0.0f),  
    vec2(-offset.x, -offset.y),
    vec2( 0.0f,   -offset.y),
    vec2( offset.x, -offset.y) 
  );
  float kernel[9] = float[]( 
    1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
    1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
    1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0  
  );
  vec3 col = vec3(0.0, 0.0, 0.0);
  for(int i = 0; i < 9; i+=1){
    col += SampleRTavg(uv + offsets[i]) * kernel[i];
  }
  return col;
}

void main() {
    vec2 uv = fuv;
    //vec2 texelSize = 1.0 / mi.resolutions.xy;
    //vec3  centerColor  = SampleRTavg(uv);
    //vec3  centerNormal = SampleNormal(uv);
    //float centerDepth  = SampleDepth(uv);
    //float centerLum    = Luminance(centerColor);
    //if (centerDepth >= 0.9999) {
    //    outColor = vec4(centerColor, 1.0);
    //    return;
    //}
    //int   passIndex  = int(mi.addinfo.y);
    //float sigmaLum   = max(mi.addinfo.z, 0.01);
    //float sigmaNorm  = max(mi.addinfo.w, 0.01);
    //float stepSize = float(1 << passIndex);
    //float variance      = EstimateVariance(uv, texelSize);
    //float adaptiveSigma = sigmaLum * (1.0 + 4.0 * sqrt(variance));
//
    //vec3  colorAccum  = vec3(0.0);
    //float weightAccum = 0.0;
//
    //for (int row = 0; row < 5; row++) {
    //    for (int col = 0; col < 5; col++) {
    //        vec2  offset    = vec2(float(col - 2), float(row - 2)) * texelSize * stepSize;
    //        vec2  sampleUV  = clamp(uv + offset, vec2(0.0), vec2(1.0));
    //        vec3  sampleColor  = SampleRTavg(sampleUV);
    //        vec3  sampleNormal = SampleNormal(sampleUV);
    //        float sampleDepth  = SampleDepth(sampleUV);
    //        float sampleLum    = Luminance(sampleColor);
    //        if (sampleDepth >= 0.9999) continue;
    //        float wKernel = KERNEL[row * 5 + col];
    //        float wNormal = NormalWeight(centerNormal, sampleNormal, sigmaNorm);
    //        float wDepth = DepthWeight(centerDepth, sampleDepth);
    //        float wLum = LuminanceWeight(centerLum, sampleLum, adaptiveSigma);
    //        float wColor = ColorWeight(centerColor, sampleColor);
    //        float w = wKernel * wNormal * wDepth * wLum * wColor;
    //        colorAccum  += sampleColor * w;
    //        weightAccum += w;
    //    }
    //}
    //vec3 denoised = colorAccum / max(weightAccum, 0.0001);
//
    //outColor = vec4(denoised, 1.0);
    outColor = vec4(SampleRT(uv), 1.0);
}