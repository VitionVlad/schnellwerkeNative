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

layout(binding = 3) uniform texture3D texTexture;

layout(binding = 4) uniform texture2DArray defferedTexture;

layout(binding = 5) uniform texture2DArray defferedDepthTexture;

layout(binding = 6) uniform texture2DArray shadowTexture;

layout(binding = 7) uniform sampler imageSampler;

layout(binding = 8) uniform sampler attachmentSampler;

layout(binding = 9) uniform texture2D noiseTexture;

layout(binding = 10) uniform sampler noiseSampler;

const float PI = 3.14159265359;

const float GOLDEN_RATIO = 0.61803398875;

const int   MAX_STEPS = 128;
const float EPS       = 1e-5;

vec3 FresnelSchlick(float cosTheta, vec3 F0){
  return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

float DistributionGGX(vec3 N, vec3 H, float roughness){
  float a  = roughness * roughness;
  float a2 = a * a;
  float NdotH = max(dot(N, H), 0.0);
  float denom = NdotH * NdotH * (a2 - 1.0) + 1.0;
  return a2 / (PI * denom * denom + 1e-6);
}

float GeometrySchlickGGX(float NdotV, float roughness){
  float r = roughness + 1.0;
  float k = (r * r) / 8.0;
  return NdotV / (NdotV * (1.0 - k) + k);
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness){
  float ggx1 = GeometrySchlickGGX(max(dot(N, V), 0.0), roughness);
  float ggx2 = GeometrySchlickGGX(max(dot(N, L), 0.0), roughness);
  return ggx1 * ggx2;
}

vec3 CosineHemisphere(vec2 xi){
  float phi = 2.0 * PI * xi.x;
  float cosTheta = sqrt(1.0 - xi.y);
  float sinTheta = sqrt(xi.y);
  return vec3(
    cos(phi) * sinTheta,
    sin(phi) * sinTheta,
    cosTheta
  );
}

vec3 ImportanceSampleGGX(vec2 Xi, float roughness){
  float a = roughness * roughness;
  float phi = 2.0 * PI * Xi.x;
  float cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a * a - 1.0) * Xi.y));
  float sinTheta = sqrt(1.0 - cosTheta * cosTheta);
  return vec3(
    cos(phi) * sinTheta,
    sin(phi) * sinTheta,
    cosTheta
  );
}

vec3 SampleGGXVNDF(vec3 V, float alpha, vec2 Xi){
  vec3 Vh = normalize(vec3(alpha * V.x, alpha * V.y, V.z));
  float lensq = Vh.x*Vh.x + Vh.y*Vh.y;
  vec3 T1 = lensq > 0.0 ? vec3(-Vh.y, Vh.x, 0.0) / sqrt(lensq) : vec3(1.0,0.0,0.0);
  vec3 T2 = cross(Vh, T1);
  float r = sqrt(Xi.x);
  float phi = 2.0 * PI * Xi.y;
  float t1 = r * cos(phi);
  float t2 = r * sin(phi);
  float s = 0.5 * (1.0 + Vh.z);
  t2 = mix(sqrt(1.0 - t1*t1), t2, s);
  vec3 Nh = t1*T1 + t2*T2 + sqrt(max(0.0, 1.0 - t1*t1 - t2*t2))*Vh;
  return normalize(vec3(alpha*Nh.x, alpha*Nh.y, max(0.0,Nh.z)));
}

mat3 CreateTBN(vec3 N){
  vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
  vec3 T = normalize(cross(up, N));
  vec3 B = cross(N, T);
  return mat3(T, B, N);
}

struct BRDFSample{
  vec3 direction;
  vec3 weight;
  float pdf;
};

BRDFSample SampleGGX(vec3 V, vec3 N, vec3 F0, float roughness, vec2 Xi){
  BRDFSample result;
  mat3 TBN = CreateTBN(N);
  vec3 Vlocal = transpose(TBN) * V;
  float alpha = max(0.001, roughness * roughness);
  vec3 Vh = normalize(vec3(alpha * Vlocal.x, alpha * Vlocal.y, Vlocal.z));
  float lensq = Vh.x * Vh.x + Vh.y * Vh.y;
  vec3 T1 = lensq > 0.0 ? vec3(-Vh.y, Vh.x, 0.0) * inversesqrt(lensq) : vec3(1.0,0.0,0.0);
  vec3 T2 = cross(Vh, T1);
  float r = sqrt(Xi.x);
  float phi = 2.0 * PI * Xi.y;
  float t1 = r * cos(phi);
  float t2 = r * sin(phi);
  float s = 0.5 * (1.0 + Vh.z);
  t2 = mix(sqrt(max(0.0, 1.0 - t1*t1)), t2, s);
  vec3 Nh = t1*T1 + t2*T2 + sqrt(max(0.0,1.0-t1*t1-t2*t2))*Vh;
  vec3 Hlocal = normalize(vec3(alpha * Nh.x, alpha * Nh.y, max(0.0, Nh.z)));
  vec3 H = normalize(TBN * Hlocal);
  result.direction = reflect(-V, H);
  float NdotV = max(dot(N,V),0.0001);
  float NdotL = max(dot(N,result.direction),0.0001);
  float NdotH = max(dot(N,H),0.0001);
  float VdotH = max(dot(V,H),0.0001);
  vec3 F = FresnelSchlick(VdotH,F0);
  float D = DistributionGGX(N,H,roughness);
  float G = GeometrySmith(N, V, result.direction, roughness);
  result.pdf = max(D * NdotH / (4.0 * VdotH), 1e-5);
  result.weight = F * G * VdotH / max(NdotV * NdotH, 1e-5);
  return result;
}

BRDFSample SampleDiffuse(vec3 albedo, vec3 N, vec2 Xi){
  BRDFSample result;
  mat3 TBN = CreateTBN(N);
  vec3 local = CosineHemisphere(Xi);
  result.direction = normalize(TBN * local);
  float NdotL = max(dot(N, result.direction), 0.0);
  result.pdf = NdotL / PI;
  result.weight = albedo;
  return result;
}

BRDFSample finalSample(vec3 V, vec3 N, vec3 albedo, float roughness, float metallic, vec3 random){
  vec3 F0 = mix(vec3(0.04), albedo, metallic);
  vec3 F = FresnelSchlick(max(dot(N,V),0.0), F0);
  float specProbability = clamp( dot(F, vec3(0.2126,0.7152,0.0722)), 0.05, 0.95);

  BRDFSample brdf;

  if(random.x < specProbability){
    brdf = SampleGGX(V, N, F0, roughness, random.yz);
    brdf.weight /= specProbability;
  }else{
    brdf = SampleDiffuse(albedo * (1.0 - metallic), N, random.yz);
    brdf.weight /= (1.0 - specProbability);
  }
  return brdf;
}


vec4 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat){
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth, 1.0);
  vec4 viewSpacePosition = inversemat * clipSpacePosition;
  viewSpacePosition.xyz /= viewSpacePosition.w;
  return viewSpacePosition;
}

vec3 WorldPosToUVDepth(vec3 worldPos, mat4 mvp) {
  vec4 clipSpace = mvp * vec4(worldPos, 1.0);
  vec3 ndc = clipSpace.xyz / clipSpace.w;
  vec2 uv = ndc.xy * 0.5 + 0.5;
  float depth = ndc.z;
  return vec3(uv, depth);
}

vec3 get_raydir(vec2 uv, float fov, inout vec3 camForward, inout vec3 camRight, inout vec3 camUp) {
  float pitch = dmi.deffrot[0].x;
  float yaw   = dmi.deffrot[0].y;

  camForward = normalize(vec3(cos(pitch) * sin(yaw), -sin(pitch), -cos(pitch) * cos(yaw)));

  vec3 worldUp  = vec3(0.0, 1.0, 0.0);
  camRight = normalize(cross(camForward, worldUp));
  camUp    = normalize(cross(camRight, camForward));

  vec2 screenPos = uv * 2.0 - 1.0;

  float aspect = mi.resolutions.x / mi.resolutions.y;
  
  float rfov = radians(fov);
  float tanHalfFov = tan(rfov * 0.5);
  
  vec3 rayDir = normalize(camForward + camRight * screenPos.x * aspect * tanHalfFov + camUp * screenPos.y * tanHalfFov);
  return rayDir;
}

vec2 RayDirToUV(vec3 rayDir, vec3 camForward, vec3 camRight, vec3 camUp, float aspect, float fov) {
  float tanHalfFov = tan(fov * 0.5);
  float d = dot(rayDir, camForward);
  float x = dot(rayDir, camRight) / (d * aspect * tanHalfFov);
  float y = dot(rayDir, camUp)    / (d * tanHalfFov);
  return vec2(x, y) * 0.5 + 0.5;
}

vec4 voxelRaycast(inout vec3 rayOrigin, vec3 rayD, out vec3 outn) {
  vec3 rayDir = vec3(rayD.x, rayD.y, rayD.z);
  vec3 color = vec3(0.0, 0.0, 0.0);
  int lastAxis = 0;
  bool vid = false;
  float tps = 0.0;

  float voxelSize = mi.rtinfo.a;
  vec3 gridMin   = mi.rtinfo.xyz;
  vec3 gridSize = mi.addinfo.yzw;
  vec3 gridMax   = gridMin + gridSize.xyz * voxelSize;
  vec3 invDir = 1.0 / (rayDir + vec3(EPS));
  vec3 t0     = (gridMin - rayOrigin) * invDir;
  vec3 t1     = (gridMax - rayOrigin) * invDir;
  vec3 tMin   = min(t0, t1);
  vec3 tMax   = max(t0, t1);

  float tEnter = max(max(tMin.x, tMin.y), tMin.z);
  float tExit  = min(min(tMax.x, tMax.y), tMax.z);
  if (tEnter > tExit || tExit < 0.0) return vec4(0.0);
  float t = max(tEnter, 0.0);
  vec3 entryPos = rayOrigin + rayDir * (t + EPS);
  vec3 localPos = (entryPos - gridMin) / voxelSize;

  ivec3 voxel = ivec3(floor(localPos));
  voxel = clamp(voxel, ivec3(0), ivec3(gridSize.xyz) - 1);
  ivec3 step = ivec3(sign(rayDir));
  vec3 tDelta = abs(voxelSize / rayDir);
  vec3 frac = localPos - floor(localPos);
  vec3 tMaxV;
  tMaxV.x = (rayDir.x > 0.0 ? (1.0 - frac.x) : frac.x) * tDelta.x;
  tMaxV.y = (rayDir.y > 0.0 ? (1.0 - frac.y) : frac.y) * tDelta.y;
  tMaxV.z = (rayDir.z > 0.0 ? (1.0 - frac.z) : frac.z) * tDelta.z;

  vec3 normal;
  for (int i = 0; i < MAX_STEPS; i++) {
    if (any(lessThan(voxel, ivec3(0))) ||
      any(greaterThanEqual(voxel, ivec3(gridSize.xyz)))) {
      break;
    }
    vec3 uvw = (vec3(voxel) + 0.5) / gridSize.xyz;
    vec4 cl = texture(sampler3D(texTexture, attachmentSampler), uvw);
    if (cl.a > 0.5) {
      if(vid){
        if (lastAxis == 0) {
          normal = vec3(-float(step.x), 0.0, 0.0);
        }else if (lastAxis == 1) {
          normal = vec3(0.0, -float(step.y), 0.0);
        }else{
          normal = vec3(0.0, 0.0, -float(step.z));
        }
        color = cl.rgb;
        rayOrigin = rayOrigin + rayDir * tps;
        outn = normal;
        break;
      }
      //break;
    }else{
      vid = true;
    }
    if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
      tps = tMaxV.x;
      tMaxV.x += tDelta.x;
      voxel.x += step.x;
      lastAxis = 0;
    } else if (tMaxV.y < tMaxV.z) {
      tps = tMaxV.y;
      tMaxV.y += tDelta.y;
      voxel.y += step.y;
      lastAxis = 1;
    } else {
      tps = tMaxV.z;
      tMaxV.z += tDelta.z;
      voxel.z += step.z;
      lastAxis = 2;
    }
  }
  return vec4(color, 1.0);
}

bool VoxelShadowRay(vec3 rayOrigin, vec3 lightPos) {
  //bool vid = false;
  vec3  toLight   = lightPos - rayOrigin;
  float lightDist = length(toLight);
  vec3  rayDir    = toLight / lightDist;
  float voxelSize = mi.rtinfo.a;

  vec3 gridMin   = mi.rtinfo.xyz;
  vec3 gridSize = mi.addinfo.yzw;
  vec3  gridMax   = gridMin + gridSize.xyz * voxelSize;

  float solidDepth = 0.0;
  for (int probe = 0; probe < 4; probe++) {
    vec3  probePos = rayOrigin + rayDir * voxelSize * float(probe);
    vec3  probeUVW = (probePos - gridMin) / (voxelSize * vec3(gridSize));
    float occ = texture(sampler3D(texTexture, attachmentSampler), probeUVW).a;
    if (occ > 0.5) solidDepth += 1.0;
  }
  rayOrigin += rayDir * voxelSize * solidDepth;

  vec3  invDir = 1.0 / (rayDir + vec3(1e-6));
  vec3  t0 = (gridMin - rayOrigin) * invDir;
  vec3  t1 = (gridMax - rayOrigin) * invDir;
  vec3  tMin   = min(t0, t1);
  vec3  tMax   = max(t0, t1);
  float tEnter = max(max(tMin.x, tMin.y), tMin.z);
  float tExit  = min(min(tMax.x, tMax.y), tMax.z);
  if (tEnter > tExit || tExit < 0.0) return false;
  float t = max(tEnter, 0.0);
  vec3  localPos = (rayOrigin + rayDir * (t + 1e-4) - gridMin) / voxelSize;
  ivec3 voxel    = clamp(ivec3(floor(localPos)), ivec3(0), ivec3(gridSize.xyz) - 1);
  ivec3 step    = ivec3(sign(rayDir));
  vec3  tDelta  = abs(voxelSize / rayDir);
  vec3  frac    = localPos - floor(localPos);
  vec3 tMaxV;
  tMaxV.x = (rayDir.x > 0.0 ? 1.0 - frac.x : frac.x) * tDelta.x;
  tMaxV.y = (rayDir.y > 0.0 ? 1.0 - frac.y : frac.y) * tDelta.y;
  tMaxV.z = (rayDir.z > 0.0 ? 1.0 - frac.z : frac.z) * tDelta.z;
  int maxSteps = int(lightDist / voxelSize) + 2;

  for (int i = 0; i < maxSteps; i++) {
    if (any(lessThan(voxel, ivec3(0))) ||
      any(greaterThanEqual(voxel, ivec3(gridSize.xyz)))) {
      return false;
    }
    float currentT = min(tMaxV.x, min(tMaxV.y, tMaxV.z));
    if (currentT * voxelSize >= lightDist) {
      return false;
    }
    vec3  uvw       = (vec3(voxel) + 0.5) / gridSize.xyz;
    float occupancy = texture(sampler3D(texTexture, attachmentSampler), uvw).a;
    if (occupancy > 0.5) {
      //if(vid){
        return true;
      //}
    }//else{
    //  vid = true;
    //}
    if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
      tMaxV.x += tDelta.x;
      voxel.x += step.x;
    } else if (tMaxV.y < tMaxV.z) {
      tMaxV.y += tDelta.y;
      voxel.y += step.y;
    } else {
      tMaxV.z += tDelta.z;
      voxel.z += step.z;
    }
  }
  return false;
}

vec3 voxelplusssrt(){
  vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, d, 1.0);

  vec3 albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;

  vec3 rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
  vec3 normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
  vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]).xyz;

  int rpx = 1;

  //if(rma.x < 0.5){
  //  rpx = 1;
  //}

  vec3 hitAlbedo = vec3(0.0, 0.0, 0.0);

  for(int j = 1; j <= rpx; j++) {
    vec3 ha = vec3(1.0, 1.0, 1.0);
    uv = vec2(fuv.x, 1.0 - fuv.y);

    normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
    albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
    rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;

    vec3 camForward;
    vec3 camRight;
    vec3 camUp;

    vec3 rayDir = get_raydir(uv, dmi.deffrot[0].w, camForward, camRight, camUp);
    ha *= albedo;

    vec2  blueNoiseUV = mod(uv*2.0 + float(mi.addinfo.x) * GOLDEN_RATIO * j + j, 1.0);
    vec4  bn = texture(sampler2D(noiseTexture, attachmentSampler), blueNoiseUV);

    BRDFSample brdf = finalSample(-rayDir, normal, albedo, rma.r, rma.g, bn.xyz);
    rayDir = brdf.direction;
    ha *= brdf.weight;

    d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
    wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]).xyz;
    uv = RayDirToUV(rayDir, camForward, camRight, camUp, mi.resolutions.x / mi.resolutions.y, radians(90.0));
    normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;

    if(uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || dot(rayDir, normal) >= 0){
      if(!VoxelShadowRay(wrldpos, smi.lightpos[0].xyz)){
        ha *= voxelRaycast(wrldpos, rayDir, normal).rgb;
        ha *= smi.lightcol[0].xyz;
      }else{
        ha *= voxelRaycast(wrldpos, rayDir, normal).rgb;
        if(!VoxelShadowRay(wrldpos, smi.lightpos[0].xyz)){
          ha *= smi.lightcol[0].xyz;
        }else{
          ha *= 0.0;
        }
      }
    }else{
      albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
      rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
      if(!VoxelShadowRay(wrldpos, smi.lightpos[0].xyz)){
        ha *= albedo;
        ha *= smi.lightcol[0].xyz;
      }else{
        ha *= albedo;
        if(!VoxelShadowRay(wrldpos, smi.lightpos[0].xyz)){
          ha *= smi.lightcol[0].xyz;
        }else{
          ha *= 0.0;
        }
      }
    }

    hitAlbedo += ha/float(rpx);
  }
  return hitAlbedo;
}

void main() {
  vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  //float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;

  vec3 result = voxelplusssrt().rgb;

  outColor = vec4(result, 1.0);
}