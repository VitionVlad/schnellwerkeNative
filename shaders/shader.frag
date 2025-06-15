#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 uv;

layout(binding = 0) uniform MeshInput {
    vec4 resolutions;
    vec4 lightinfo;
    mat4 model;
    vec4 addinfo;
} mi;

layout(binding = 1) uniform ShadowMatricesInput {
    mat4 shadowViews[100];
    vec4 lightpos[100];
    vec4 lightcol[100];
} smi;

layout(binding = 2) uniform DefferedMatricesInput {
    mat4 defferedViews[10];
    vec4 deffpos[10];
    vec4 deffcol[10];
} dmi;

layout(binding = 3) uniform sampler2DArray texSampler;

layout(binding = 4) uniform sampler2DArray defferedSampler;

layout(binding = 5) uniform sampler2DArray defferedDepthSampler;

layout(binding = 6) uniform sampler2DArray shadowSampler;

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
        if (proj.z - bias > texture(shadowSampler, vec3(proj.x + offset.x, 1.0 - proj.y + offset.y, i)).r){
            lv = 1.0;
        }
        if (!(proj.x > 1.0 || proj.x < 0.0 || proj.y > 1.0 || proj.y < 0.0 || proj.z > 1.0 || proj.z < -1.0)){
          visibility += lv;
        }
      }
    }
  }
  return visibility / 9.0;
}

vec3 PBR(vec3 norm, vec3 albedo, float shadow, float metallic, float roughness, float ao, vec3 WorldPos){
  vec3 N = normalize(norm);
  vec3 V = normalize(dmi.deffpos[0].xyz - WorldPos);
  vec3 F0 = vec3(0.04); 
  F0 = mix(F0, albedo, metallic);
  vec3 Lo = vec3(0.0);
  for(int i = 0; i < mi.lightinfo.x; i++) {
    vec3 L = smi.lightpos[i].xyz;
    vec3 H = normalize(V + L);
    if (smi.lightpos[i].w != 0.0) {
      L = normalize(smi.lightpos[i].xyz - WorldPos);
      H = normalize(V + L);
    }
    float distance    = length(smi.lightpos[i].xyz - WorldPos);
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
    Lo += (kD * albedo / PI + specular) * radiance * NdotL; 
  }
  vec3 ambient = vec3(0.0001) * albedo * ao;
  vec3 color = ambient + shadow * Lo;
  color = color / (color + vec3(1.0));
  color = pow(color, vec3(1.0/2.2));  
  return color;
}

void main() {
    vec3 albedo = pow(texture(defferedSampler, vec3(uv, 0)).rgb, vec3(2.2));
    vec3 rma = texture(defferedSampler, vec3(uv, 1)).rgb;
    vec3 normal = texture(defferedSampler, vec3(uv, 2)).rgb;
    vec3 wrldpos = texture(defferedSampler, vec3(uv, 3)).rgb;
    vec4 op = vec4(PBR(normal, albedo, 1.0 - shcalc(wrldpos, 0.001), rma.y, rma.x, 1.0, wrldpos), 1.0);
    //vec4 op = vec4(normal, 1.0);

    outColor = op;
    //if (texture(defferedDepthSampler, vec3(uv, 1)).r < texture(defferedDepthSampler, vec3(uv, 0)).r){
    //    outColor = mix(vec4(texture(defferedSampler, vec3(uv, 4)).rgb, 1.0), op, 0.5);
    //}
}