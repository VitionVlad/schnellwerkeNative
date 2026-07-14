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

const float PI = 3.14159265359;

const vec3 EMISSIVE_COLOR = vec3(1.0, 0.85, 0.55) * 6.0;

const int   MAX_STEPS = 128;
const float EPS       = 1e-5;

float rand(vec2 co){
  return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

vec3 randomOnSphere(vec2 uv, int j) {
  float seed = fract(sin(dot(uv, vec2(127.1*j, 311.7))) * 43758.5453 + mi.addinfo.x*j);
	vec3 rand = vec3(rand(vec2(seed * uv.x, -seed * uv.y)), rand(vec2(seed * uv.y, seed * uv.x)), rand(vec2(1.0)-vec2(seed * uv.x, -seed * uv.y)));
	float theta = rand.x * 2.0 * 3.14159265;
	float v = rand.y;
	float phi = acos(2.0 * v - 1.0);
	float r = pow(rand.z, 1.0 / 3.0);
	float x = r * sin(phi) * cos(theta);
	float y = r * sin(phi) * sin(theta);
	float z = r * cos(phi);
	return vec3(x, y, z);
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

vec3 SSRT(){
  vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, d, 1.0);
  //d = LinearizeDepth(d);

  vec3 albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;

  vec3 rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
  vec3 normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
  vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]).xyz;

  vec3 hitAlbedo = vec3(0.0, 0.0, 0.0);

  for(int j = 1; j <= 2; j++) {
    vec3 ha = vec3(1.0, 1.0, 1.0);
    uv = vec2(fuv.x, 1.0 - fuv.y);

    normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
    albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
    rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
    d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;

    vec3 camForward;
    vec3 camRight;
    vec3 camUp;

    //bool lt = false;

    //vec3 or = dmi.deffpos[0].xyz;
    vec3 rayDir = get_raydir(uv, 90.0, camForward, camRight, camUp);
    
    ha *= albedo;

    //ha *= ScreenSpaceShadow(wrldpos, normal, smi.lightpos[0].xyz, dmi.defferedMVP[0], 0.3);

    rayDir = reflect(rayDir, normal);
    vec3 rand = randomOnSphere(uv * mi.addinfo.x, j);
    vec3 diff = normalize(rand * dot(rand, normal));
    rayDir = mix(rayDir, diff, rma.r);

    //rayDir = reflect(rayDir, normal);
    //vec3 rand = randomOnSphere(uv * mi.addinfo.x, j);
    //vec3 diff = normalize(rand * dot(rand, normal));
    //rayDir = mix(rayDir, diff, rma.r);

    //uv = RayDirToUV(rayDir, camForward, camRight, camUp, mi.resolutions.x / mi.resolutions.y, radians(90.0));

    //d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
    //wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]).xyz;
    //normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
    //albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
    //rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;

    if(uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || dot(rayDir, normal) <= 0){
    }else{
      for(int j = 0; j != 64; j++){
        wrldpos += rayDir;
        vec3 uvd = WorldPosToUVDepth(wrldpos, dmi.defferedMVP[0]);
        d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uvd.xy, 0)).r;
        float delta = uvd.z - d;
        if(delta < 0.1){
          uv = uvd.xy;
          d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
          wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]).xyz;
          normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
          albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
          rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
          ha *= albedo;
          break;
        }
      }
    }

    hitAlbedo += ha/2.0;
  }
  return hitAlbedo;
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

  vec3 hitAlbedo = vec3(0.0, 0.0, 0.0);

  bool lthit = false;

  for(int j = 1; j <= 1; j++) {
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

    rayDir = reflect(rayDir, normal);
    vec3 rand = randomOnSphere(uv * mi.addinfo.x, j);
    vec3 diff = normalize(rand * dot(rand, normal));
    rayDir = mix(rayDir, diff, rma.r);

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

    hitAlbedo += ha/2.0;
  }
  return hitAlbedo;
}

void main() {
  //vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  //float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;

  vec3 result = voxelplusssrt().rgb;

  outColor = vec4(result, 1.0);
}