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

layout(binding = 4) uniform texture2DArray defferedTexture;

layout(binding = 5) uniform texture2DArray defferedDepthTexture;

layout(binding = 6) uniform texture2DArray shadowTexture;

layout(binding = 7) uniform sampler imageSampler;

layout(binding = 8) uniform sampler attachmentSampler;

const float PI = 3.14159265359;

float DistributionGGX(vec3 N, vec3 H, float roughness){
  float a      = roughness*roughness;
  float a2     = a*a;
  float NdotH  = max(dot(N, H), 0.0);
  float NdotH2 = NdotH*NdotH;
  float num   = a2;
  float denom = (NdotH2 * (a2 - 1.0) + 1.0);
  denom = PI * denom * denom;
  return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness){
  float r = (roughness + 1.0);
  float k = (r*r) / 8.0;
  float num   = NdotV;
  float denom = NdotV * (1.0 - k) + k;
  return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness){
  float NdotV = max(dot(N, V), 0.0);
  float NdotL = max(dot(N, L), 0.0);
  float ggx2  = GeometrySchlickGGX(NdotV, roughness);
  float ggx1  = GeometrySchlickGGX(NdotL, roughness);
  return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0){
  return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float shcalc(vec3 WorldPos, float bias){
  float visibility = 0.0;
  for (int i = 0; i < mi.lightinfo.x; i++) {
    vec4 smv = smi.shadowViews[i] * vec4(WorldPos, 1.0);
    vec3 proj = vec3((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
    float oneOverShadowDepthTextureSize = 1.0 / mi.resolutions.z;
    for (int y = -1; y <= 1; y++) {
      for (int x = -1; x <= 1; x++) {
        vec2 offset = vec2(vec2(x, y)) * oneOverShadowDepthTextureSize;
        float lv = 0.0;
        if (proj.z - bias < texture(sampler2DArray(shadowTexture, attachmentSampler), vec3(proj.x + offset.x, 1.0 - proj.y + offset.y, i)).r){
          lv = 1.0;
        }
        if (!(proj.x > 0.99 || proj.x < 0.001 || proj.y > 0.99 || proj.y < 0.001 || proj.z > 1.0 || proj.z < -1.0)){
          visibility += lv;
        }
      }
    }
  }
  return visibility / 9.0;
}

float shcalcpl(vec3 WorldPos, float bias, int i){
  float visibility = 0.0;
  vec4 smv = smi.shadowViews[i] * vec4(WorldPos, 1.0);
  vec3 proj = vec3((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
  float oneOverShadowDepthTextureSize = 1.0 / mi.resolutions.z;
  for (int y = -1; y <= 1; y++) {
    for (int x = -1; x <= 1; x++) {
      vec2 offset = vec2(vec2(x, y)) * oneOverShadowDepthTextureSize;
      float lv = 0.0;
      if (proj.z - bias < texture(sampler2DArray(shadowTexture, attachmentSampler), vec3(proj.x + offset.x, 1.0 - proj.y + offset.y, i)).r){
        lv = 1.0;
      }
      if (!(proj.x > 0.99 || proj.x < 0.001 || proj.y > 0.99 || proj.y < 0.001 || proj.z > 1.0 || proj.z < -1.0)){
        visibility += lv;
      }
    }
  }
  return visibility / 9.0;
}

vec3 PBR(vec3 norm, vec3 albedo, float metallic, float roughness, float ao, vec3 WorldPos){
  vec3 N = normalize(norm);
  vec3 V = normalize(dmi.deffpos[0].xyz - WorldPos);
  vec3 F0 = vec3(0.04); 
  F0 = mix(F0, albedo, metallic);
  vec3 Lo = vec3(0.0);
  for(int i = 0; i < mi.lightinfo.x; i++) {
    vec3 L = normalize(smi.lightpos[i].xyz);
    vec3 H = normalize(V + L);
    float distance    = 1.0;
    if (smi.lightpos[i].w == 1.0) {
      L = normalize(smi.lightpos[i].xyz - WorldPos);
      H = normalize(V + L);
      distance    = length(smi.lightpos[i].xyz - WorldPos);
    }
    float attenuation = 1.0 / (distance * distance); 
    vec3 radiance     = (smi.lightcol[i].xyz) * attenuation;    
    float NDF = DistributionGGX(N, H, roughness);        
    float G   = GeometrySmith(N, V, L, roughness);      
    vec3 F   = fresnelSchlick(max(dot(H, V), 0.0), F0);       
    vec3 kS = F;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;	  
    vec3 numerator    = NDF * G * F;
    float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
    vec3 specular     = numerator / denominator;  
    float NdotL = max(dot(N, L), 0.0);
    Lo += (kD * albedo / PI + specular) * radiance * NdotL * shcalcpl(WorldPos, 0.0, i); 
  }
  vec3 ambient = vec3(0.00001) * albedo * ao;
  vec3 color = ambient + Lo;
  color = color / (color + vec3(1.0));
  color = pow(color, vec3(1.0/2.2));  
  return color;
}

void getCameraBasis(vec3 eulerAngles, out vec3 forward, out vec3 right, out vec3 up) {
    float pitch = -eulerAngles.x;
    float yaw = -eulerAngles.y;
    float roll = eulerAngles.z;
    forward.x = cos(pitch) * sin(yaw);
    forward.y = sin(pitch);
    forward.z = cos(pitch) * cos(yaw);
    forward = normalize(forward);
    right.x = sin(yaw - 1.5708);
    right.y = 0.0;
    right.z = cos(yaw - 1.5708);
    right = normalize(right);
    up = normalize(cross(right, forward));
}

vec3 nightSkyFog(vec2 uv, vec3 cameraPos, vec3 cameraEuler, float time, bool rng) {
  vec3 forward, right, up;
  getCameraBasis(cameraEuler, forward, right, up);
  vec2 ndc = uv * 2.0 - 1.0;
  float fovScale = 1.0;
  vec3 rayDir = normalize(
      forward +
      ndc.x * fovScale * right +
      ndc.y * fovScale * up
  );
  vec3 samplePos = cameraPos + rayDir * 20.0;
  float fogDriftSpeed = -20.2;
  float drift = (cameraPos.z + time * fogDriftSpeed) * 0.05;
  float noise = sin(dot(samplePos.xz, vec2(0.05, 0.05)) + drift);
  noise = noise * 0.5 + 0.5;
  float heightFog = smoothstep(50.0, 0.0, samplePos.y);
  float distFog = smoothstep(5.0, 30.0, length(samplePos - cameraPos));
  float fogAmount = noise * heightFog * distFog * 0.5;
  vec3 colmx1 = vec3(0.002, 0.002, 0.005);
  vec3 colmx2 = vec3(0.005, 0.005, 0.01);
  vec3 fogColor = mix(colmx1, colmx2, noise);

  float tGround = -(cameraPos.y) / rayDir.y;
  bool hitGround = (rayDir.y < -0.001) && (tGround > 0.0);
  if (hitGround && rng) {
    vec3 groundPos = vec3(0.0, cameraPos.y, cameraPos.z) + rayDir * tGround;
    float groundDist = length(groundPos - vec3(0.0, cameraPos.y, cameraPos.z));
    if (groundDist <= 15.0) {
        vec2 groundUV = groundPos.xz * 0.2;
        groundUV.y -= time * 2.0;
        float groundPattern = cos(groundUV.y) * 0.5 + 0.5;
        groundPattern = pow(groundPattern, 3.0);
        vec3 groundColor = mix(vec3(0.01, 0.03, 0.01), vec3(0.035, 0.025, 0.02), min(max(groundPattern, 0.0), 1.0));
        float groundFogFactor = smoothstep(10.0, 0.0, groundDist);
        fogColor = mix(fogColor, groundColor, groundFogFactor);
    }
  }

  return fogColor * fogAmount;
}

vec3 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat){
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth, 1.0);
  vec4 viewSpacePosition = inversemat * clipSpacePosition;
  viewSpacePosition /= viewSpacePosition.w;
  return viewSpacePosition.xyz;
}

float near = 0.1; 
float far  = 100.0; 

float LinearizeDepth(float d) {         
    return near * far / (far + d * (far - near));
}

void main() {
  vec2 uv = fuv;
  float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  //d = LinearizeDepth(d);

  vec3 albedo = pow(texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb, vec3(2.2));

  vec3 rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
  vec3 normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb; 
  vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

  vec4 op = vec4(PBR(normal, albedo, rma.y, rma.x, 1.0, wrldpos), 1.0);

  if(rma.y <= 0.1 && rma.x <= 0.1){
    op = vec4(albedo, 1.0);
  }

  //float mxpw = smoothstep(10.0, 20.0, distance(mi.addinfo.yz, wrldpos.xz));

  //op = mix(op, vec4(smi.lightcol[0].xyz, 1.0), mxpw);

  //op = mix(op, vec4(0.0, 0.0, 0.0, 1.0), mi.addinfo.x);

  outColor = op;

  //outColor = vec4(texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb, 1.0);

  //float camd = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  //float camd2 = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 1)).r;

  //if(camd2 < camd){
  //  vec3 c2wp = WorldPosFromDepth(camd2, uv, dmi.defferedMVPInverse[1]);
  //  vec4 gf = vec4(PBR(texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 5)).rgb, vec3(0.1), shcalc(c2wp, 0.0), 0.1, 0.1, 1.0, c2wp), 1.0);
  //  op = mix(op, gf, gf.r);
  //}

  //outColor = vec4(vec3(texture(sampler2DArray(shadowTexture, attachmentSampler), vec3(uv, 0)).r), 1.0);

  //outColor = vec4(WorldPosFromDepth(texture(defferedDepthSampler, vec3(uv, 0)).r, uv, dmi.defferedMVPInverse[0]), 1.0);
}