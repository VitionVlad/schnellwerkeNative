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

layout(binding = 3) uniform texture3D texTexture;

layout(binding = 4) uniform texture2DArray defferedTexture;

layout(binding = 5) uniform texture2DArray defferedDepthTexture;

layout(binding = 6) uniform texture2DArray shadowTexture;

layout(binding = 7) uniform sampler imageSampler;

layout(binding = 8) uniform sampler attachmentSampler;

const float PI = 3.14159265359;

const vec3 EMISSIVE_COLOR = vec3(1.0, 0.85, 0.55) * 6.0;

const int   MAX_STEPS = 512;
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

vec3 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat){
  vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth, 1.0);
  vec4 viewSpacePosition = inversemat * clipSpacePosition;
  viewSpacePosition /= viewSpacePosition.w;
  return viewSpacePosition.xyz;
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
  vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

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
    for(int i = 0; i < 2; i++){
      //if(albedo.b > albedo.r && albedo.b > albedo.g){
      //  ha *= albedo * 6.0;
      //  lt = true;
      //  break;
      //}
      ha *= albedo;

      rayDir = reflect(rayDir, normal);
      vec3 rand = randomOnSphere(uv * mi.addinfo.x, j);
      //vec3 spec = reflect(rayDir, normal);
      vec3 diff = normalize(rand * dot(rand, normal));
      //ro += rayDir * vec3(minIt.x - 0.001);
      rayDir = mix(rayDir, diff, rma.r);

      uv = RayDirToUV(rayDir, camForward, camRight, camUp, mi.resolutions.x / mi.resolutions.y, radians(90.0));

      d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
      wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

      if(uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0){
        //ha *= vec3(0.0, 0.0, 0.0);
        break;
      }
      normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
      albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
      rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
      //or = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);
    }
    //if(lt){
      hitAlbedo += ha/2.0;
    //}
  }
  return hitAlbedo;
}

vec4 voxelRaycast(vec3 rayOrigin, vec3 rayDir) {
    float voxelSize = 0.5;
    vec3 gridMin   = vec3(-56.0, -4.0, -40.0);
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
    ivec3 step    = ivec3(sign(rayDir));
    vec3 tDelta = abs(voxelSize / rayDir);
    vec3 frac    = localPos - floor(localPos);
    vec3 tMaxV;
    tMaxV.x = (rayDir.x > 0.0 ? (1.0 - frac.x) : frac.x) * tDelta.x;
    tMaxV.y = (rayDir.y > 0.0 ? (1.0 - frac.y) : frac.y) * tDelta.y;
    tMaxV.z = (rayDir.z > 0.0 ? (1.0 - frac.z) : frac.z) * tDelta.z;
    int lastAxis = 0;
    for (int i = 0; i < MAX_STEPS; i++) {
        if (any(lessThan(voxel, ivec3(0))) ||
            any(greaterThanEqual(voxel, ivec3(gridSize.xyz)))) {
            break;
        }
        vec3 uvw = (vec3(voxel) + 0.5) / gridSize.xyz;
        float occupancy = texture(sampler3D(texTexture, imageSampler), uvw).r;

        if (occupancy > 0.5) {
            vec3 normal;
            if      (lastAxis == 0) normal = vec3(-float(step.x), 0.0, 0.0);
            else if (lastAxis == 1) normal = vec3(0.0, -float(step.y), 0.0);
            else                    normal = vec3(0.0, 0.0, -float(step.z));

            vec3  lightDir = normalize(vec3(0.5, 1.0, 0.3));
            float diffuse  = max(dot(normal, lightDir), 0.0);

            float ambient  = 0.15;
            vec3 axisColor;
            if      (lastAxis == 0) axisColor = vec3(1.0, 0.5, 0.5);
            else if (lastAxis == 1) axisColor = vec3(0.5, 1.0, 0.5);
            else                    axisColor = vec3(0.5, 0.5, 1.0);

            vec3 color = axisColor * (diffuse + ambient);
            return vec4(color, 1.0);
        }
        if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
            tMaxV.x  += tDelta.x;
            voxel.x  += step.x;
            lastAxis  = 0;
        } else if (tMaxV.y < tMaxV.z) {
            tMaxV.y  += tDelta.y;
            voxel.y  += step.y;
            lastAxis  = 1;
        } else {
            tMaxV.z  += tDelta.z;
            voxel.z  += step.z;
            lastAxis  = 2;
        }
    }
    return vec4(0.05, 0.05, 0.08, 0.0);
}

void main() {
  vec2 uv = vec2(fuv.x, 1.0 - fuv.y);
  //float d = texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
  //vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, d, 1.0);
  //d = LinearizeDepth(d);

  //vec3 albedo = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;

  //vec3 rma = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
  //vec3 normal = texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
  //vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

  vec3 camForward;
  vec3 camRight;
  vec3 camUp;

  vec3 rayDir = get_raydir(uv, 90.0, camForward, camRight, camUp);

  vec4 result = voxelRaycast(dmi.deffpos[0].xyz, rayDir);

  outColor = vec4(result.rgb, 1.0);
}