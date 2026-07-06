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

const vec3  EMISSIVE_COLOR  = vec3(1.0, 0.85, 0.55) * 6.0;

const vec3  DIR_LIGHT_DIR       = normalize(vec3(0.5, 1.0, 0.3));
const vec3  DIR_LIGHT_COLOR     = vec3(1.0, 0.95, 0.85);  // slightly warm white
const float DIR_LIGHT_INTENSITY = 3.0;

vec3 DirectionalLight(vec3 normal, vec3 viewDir, vec3 albedo, float roughness) {
  float NdotL   = max(dot(normal, DIR_LIGHT_DIR), 0.0);
  vec3  diffuse = albedo * DIR_LIGHT_COLOR * DIR_LIGHT_INTENSITY * NdotL;

  vec3  halfVec = normalize(DIR_LIGHT_DIR + (-viewDir));
  float NdotH   = max(dot(normal, halfVec), 0.0);
  float gloss   = max((1.0 - roughness) * 128.0, 1.0);
  vec3  specular = DIR_LIGHT_COLOR * DIR_LIGHT_INTENSITY
                 * pow(NdotH, gloss)
                 * (1.0 - roughness); 

  return diffuse + specular;
}

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

  camForward = normalize(vec3(
      cos(pitch) * sin(yaw),
      -sin(pitch),
     -cos(pitch) * cos(yaw)
  ));

  vec3 worldUp  = vec3(0.0, 1.0, 0.0);
  camRight = normalize(cross(camForward, worldUp));
  camUp    = normalize(cross(camRight, camForward));

  vec2 screenPos = uv * 2.0 - 1.0;

  float aspect = mi.resolutions.x / mi.resolutions.y;
  
  float rfov    = radians(fov);
  float tanHalfFov = tan(rfov * 0.5);
  
  vec3 rayDir = normalize(
      camForward
      + camRight * screenPos.x * aspect * tanHalfFov
      + camUp    * screenPos.y * tanHalfFov
  );
  return rayDir;
}

vec2 RayDirToUV(vec3 rayDir, vec3 camForward, vec3 camRight, vec3 camUp, float aspect, float fov) {
  float tanHalfFov = tan(fov * 0.5);
  float d = dot(rayDir, camForward);
  float x = dot(rayDir, camRight) / (d * aspect   * tanHalfFov);
  float y = dot(rayDir, camUp)    / (d             * tanHalfFov);
  return vec2(x, y) * 0.5 + 0.5;
}

void main() {
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

  outColor = vec4(hitAlbedo, 1.0);
}