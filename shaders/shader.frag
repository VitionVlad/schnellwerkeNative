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

float shcalcpld(vec3 WorldPos, float bias, int i){
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
      visibility += lv;
    }
  }
  return visibility / 9.0;
}

vec3 PBR(vec3 norm, vec3 albedo, float metallic, float roughness, float ao, vec3 WorldPos){
  vec3 N = normalize(norm);
  vec3 V = normalize(dmi.deffpos[0].xyz - WorldPos - vec3(1.0));
  vec3 F0 = vec3(0.04); 
  F0 = mix(F0, albedo, metallic);
  vec3 Lo = vec3(0.0);
  for(int i = 0; i < mi.lightinfo.x; i++) {
    vec3 L = normalize(-smi.lightpos[i].xyz);
    vec3 H = normalize(V + L);
    float distance = 1.0;
    if (smi.lightpos[i].w == 1.0) {
      V = normalize(dmi.deffpos[0].xyz - WorldPos);
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
    if(smi.lightpos[i].w == 1.0){
      Lo += (kD * albedo / PI + specular) * radiance * NdotL * max(shcalcpl(WorldPos, 0.0, i), 0.001); 
    }else{
      Lo += (kD * albedo / PI + specular) * radiance * NdotL * max(shcalcpld(WorldPos, 0.0, i), 0.001); 
    }
  }
  vec3 ambient = vec3(0.001) * albedo * ao;
  vec3 color = ambient + Lo;
  color = color / (color + vec3(1.0));
  color = pow(color, vec3(1.0/2.2));  
  return color;
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

  vec4 op = vec4(PBR(normal, albedo, rma.x, rma.y, 1.0, wrldpos), 1.0);

  float dst = smoothstep(0.0, 30.0, distance(mi.addinfo.yz, wrldpos.xz));

  op = mix(vec4(smi.lightcol[0].xyz, 1.0), op, 1.0-max(min(dst, 1.0), 0.0));

  //op = vec4(abs(normal), 1.0);

  //if(rma.y <= 0.1 && rma.x <= 0.1){
  //  op = vec4(albedo, 1.0);
  //}

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