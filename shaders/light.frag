#version 450

layout(location = 0) out vec4 outColor;
layout(location = 0) in  vec2 fuv;

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

layout(binding = 3)  uniform texture3D      texTexture;
layout(binding = 4)  uniform texture2DArray defferedTexture;
layout(binding = 5)  uniform texture2DArray defferedDepthTexture;
layout(binding = 6)  uniform texture2DArray shadowTexture;
layout(binding = 7)  uniform sampler        imageSampler;
layout(binding = 8)  uniform sampler        attachmentSampler;
layout(binding = 9)  uniform texture2D      noiseTexture;
layout(binding = 10) uniform sampler        noiseSampler;

const float PI           = 3.14159265359;
const float GOLDEN_RATIO = 0.61803398875;
const int   MAX_STEPS    = 128;
const float EPS          = 1e-5;
const float SS_THICKNESS = 0.3;
const vec3  SKY_COLOR    = vec3(0.02, 0.02, 0.05);
const float FIREFLY_CAP  = 10.0;                  

vec3 SampleAlbedo(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 0)).rgb;
}
vec3 SampleRMA(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 1)).rgb;
}
vec3 SampleNormal(vec2 uv) {
    return texture(sampler2DArray(defferedTexture, attachmentSampler), vec3(uv, 2)).rgb;
}
float SampleDepth(vec2 uv) {
    return texture(sampler2DArray(defferedDepthTexture, attachmentSampler), vec3(uv, 0)).r;
}

vec3 WorldPosFromDepth(float depth, vec2 uv, mat4 inversemat) {
    vec4 clip  = vec4(uv * 2.0 - 1.0, depth, 1.0);
    vec4 world = inversemat * clip;
    return world.xyz / world.w;
}

vec3 WorldPosToUVDepth(vec3 worldPos, mat4 mvp) {
    vec4 clip = mvp * vec4(worldPos, 1.0);
    vec3 ndc  = clip.xyz / clip.w;
    return vec3(ndc.xy * 0.5 + 0.5, ndc.z);
}

vec3 get_raydir(vec2 uv, float fov, inout vec3 camForward, inout vec3 camRight, inout vec3 camUp) {
    float pitch = dmi.deffrot[0].x;
    float yaw   = dmi.deffrot[0].y;

    camForward = normalize(vec3(
        cos(pitch) * sin(yaw),
       -sin(pitch),
       -cos(pitch) * cos(yaw)
    ));

    vec3 worldUp = vec3(0.0, 1.0, 0.0);
    camRight = normalize(cross(camForward, worldUp));
    camUp = normalize(cross(camRight, camForward));

    vec2  screen = uv * 2.0 - 1.0;
    float aspect = mi.resolutions.x / mi.resolutions.y;
    float tanHalfFov = tan(radians(fov) * 0.5);

    return normalize(camForward + camRight * screen.x * aspect * tanHalfFov + camUp * screen.y * tanHalfFov);
}

vec2 RayDirToUV(vec3 rayDir, vec3 camForward, vec3 camRight, vec3 camUp, float aspect, float fovDeg) {
    float tanHalfFov = tan(radians(fovDeg) * 0.5);
    float d = dot(rayDir, camForward);
    float x = dot(rayDir, camRight) / (d * aspect   * tanHalfFov);
    float y = dot(rayDir, camUp)    / (d             * tanHalfFov);
    return vec2(x, y) * 0.5 + 0.5;
}

// ── PBR ───────────────────────────────────────────────────────────────────────

vec3 FresnelSchlick(float cosTheta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

float DistributionGGX(vec3 N, vec3 H, float roughness) {
    float a     = roughness * roughness;
    float a2    = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float denom = NdotH * NdotH * (a2 - 1.0) + 1.0;
    return a2 / (PI * denom * denom + 1e-6);
}

float GeometrySchlickGGX(float NdotV, float roughness) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;
    return NdotV / (NdotV * (1.0 - k) + k);
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
    return GeometrySchlickGGX(max(dot(N, V), 0.0), roughness)
         * GeometrySchlickGGX(max(dot(N, L), 0.0), roughness);
}

vec3 CosineHemisphere(vec2 xi) {
    float phi      = 2.0 * PI * xi.x;
    float cosTheta = sqrt(1.0 - xi.y);
    float sinTheta = sqrt(xi.y);
    return vec3(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);
}

mat3 CreateTBN(vec3 N) {
    vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
    vec3 T  = normalize(cross(up, N));
    vec3 B  = cross(N, T);
    return mat3(T, B, N);
}

struct BRDFSample {
    vec3  direction;
    vec3  weight;
    float pdf;
};

BRDFSample SampleGGX(vec3 V, vec3 N, vec3 F0, float roughness, vec2 Xi) {
    BRDFSample result;
    mat3  TBN    = CreateTBN(N);
    vec3  Vlocal = transpose(TBN) * V;
    float alpha  = max(0.001, roughness * roughness);

    vec3 Vh   = normalize(vec3(alpha * Vlocal.x, alpha * Vlocal.y, Vlocal.z));
    float lensq = Vh.x * Vh.x + Vh.y * Vh.y;
    vec3 T1   = lensq > 0.0 ? vec3(-Vh.y, Vh.x, 0.0) * inversesqrt(lensq) : vec3(1.0, 0.0, 0.0);
    vec3 T2   = cross(Vh, T1);

    float r   = sqrt(Xi.x);
    float phi = 2.0 * PI * Xi.y;
    float t1  = r * cos(phi);
    float t2  = r * sin(phi);
    float s   = 0.5 * (1.0 + Vh.z);
    t2 = mix(sqrt(max(0.0, 1.0 - t1*t1)), t2, s);

    vec3 Nh = t1*T1 + t2*T2 + sqrt(max(0.0, 1.0 - t1*t1 - t2*t2)) * Vh;
    vec3 Hlocal = normalize(vec3(alpha * Nh.x, alpha * Nh.y, max(0.0, Nh.z)));
    vec3 H = normalize(TBN * Hlocal);

    result.direction = reflect(-V, H);

    float NdotV = max(dot(N, V),  0.0001);
    float NdotL = max(dot(N, result.direction), 0.0001);
    float NdotH = max(dot(N, H), 0.0001);
    float VdotH = max(dot(V, H), 0.0001);

    vec3  F = FresnelSchlick(VdotH, F0);
    float D = DistributionGGX(N, H, roughness);
    float G = GeometrySmith(N, V, result.direction, roughness);

    result.pdf    = max(D * NdotH / (4.0 * VdotH), 1e-5);
    result.weight = F * G * VdotH / max(NdotV * NdotH, 1e-5);
    return result;
}

BRDFSample SampleDiffuse(vec3 albedo, vec3 N, vec2 Xi) {
    BRDFSample result;
    mat3 TBN       = CreateTBN(N);
    vec3 local     = CosineHemisphere(Xi);
    result.direction = normalize(TBN * local);
    float NdotL    = max(dot(N, result.direction), 0.0);
    result.pdf     = NdotL / PI;
    result.weight  = albedo;
    return result;
}

BRDFSample finalSample(vec3 V, vec3 N, vec3 albedo,
                        float roughness, float metallic, vec3 rnd) {
    vec3 F0   = mix(vec3(0.04), albedo, metallic);
    vec3 F    = FresnelSchlick(max(dot(N, V), 0.0), F0);
    float specP = clamp(dot(F, vec3(0.2126, 0.7152, 0.0722)), 0.05, 0.95);

    BRDFSample brdf;
    if (rnd.x < specP) {
        brdf = SampleGGX(V, N, F0, roughness, rnd.yz);
        brdf.weight /= specP;
    } else {
        brdf = SampleDiffuse(albedo * (1.0 - metallic), N, rnd.yz);
        brdf.weight /= (1.0 - specP);
    }
    return brdf;
}

vec4 voxelRaycast(inout vec3 rayOrigin, vec3 rayDir, out vec3 outn) {
    float voxelSize = mi.rtinfo.w;
    vec3  gridMin   = mi.rtinfo.xyz;
    vec3  gridSize  = mi.addinfo.yzw;
    vec3  gridMax   = gridMin + gridSize * voxelSize;

    outn = vec3(0.0, 1.0, 0.0);

    vec3  invDir = 1.0 / (rayDir + vec3(EPS));
    vec3  t0     = (gridMin - rayOrigin) * invDir;
    vec3  t1     = (gridMax - rayOrigin) * invDir;
    vec3  tMinV  = min(t0, t1);
    vec3  tMaxV2 = max(t0, t1);

    float tEnter = max(max(tMinV.x, tMinV.y), tMinV.z);
    float tExit  = min(min(tMaxV2.x, tMaxV2.y), tMaxV2.z);
    if (tEnter > tExit || tExit < 0.0) return vec4(0.0); // miss

    float t        = max(tEnter, 0.0);
    vec3  entryPos = rayOrigin + rayDir * (t + EPS);
    vec3  localPos = (entryPos - gridMin) / voxelSize;

    ivec3 voxel  = clamp(ivec3(floor(localPos)), ivec3(0), ivec3(gridSize) - 1);
    ivec3 vstep  = ivec3(sign(rayDir));
    vec3  tDelta = abs(voxelSize / rayDir);
    vec3  frac   = localPos - floor(localPos);

    vec3 tMaxV;
    tMaxV.x = (rayDir.x > 0.0 ? 1.0 - frac.x : frac.x) * tDelta.x;
    tMaxV.y = (rayDir.y > 0.0 ? 1.0 - frac.y : frac.y) * tDelta.y;
    tMaxV.z = (rayDir.z > 0.0 ? 1.0 - frac.z : frac.z) * tDelta.z;

    int  lastAxis = 0;
    bool sawEmpty = false;
    float tps     = 0.0;

    for (int i = 0; i < MAX_STEPS; i++) {
        if (any(lessThan(voxel, ivec3(0))) ||
            any(greaterThanEqual(voxel, ivec3(gridSize)))) {
            return vec4(0.0);
        }
        vec3 uvw = (vec3(voxel) + 0.5) / gridSize;
        vec4 cl  = texture(sampler3D(texTexture, attachmentSampler), uvw);
        if (cl.a > 0.5) {
            if (sawEmpty) {
                vec3 n;
                if (lastAxis == 0) n = vec3(-float(vstep.x), 0.0, 0.0);
                else if (lastAxis == 1) n = vec3(0.0, -float(vstep.y), 0.0);
                else n = vec3(0.0, 0.0, -float(vstep.z));
                outn      = n;
                rayOrigin = rayOrigin + rayDir * tps;
                return vec4(cl.rgb, 1.0);
            }
        } else {
            sawEmpty = true;
        }
        if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
            tps      = tMaxV.x; tMaxV.x += tDelta.x;
            voxel.x += vstep.x; lastAxis  = 0;
        } else if (tMaxV.y < tMaxV.z) {
            tps      = tMaxV.y; tMaxV.y += tDelta.y;
            voxel.y += vstep.y; lastAxis  = 1;
        } else {
            tps      = tMaxV.z; tMaxV.z += tDelta.z;
            voxel.z += vstep.z; lastAxis  = 2;
        }
    }

    return vec4(0.0);
}

bool VoxelShadowRay(vec3 rayOrigin, vec3 lightPos) {
    vec3  toLight   = lightPos - rayOrigin;
    float lightDist = length(toLight);
    vec3  rayDir    = toLight / lightDist;
    float voxelSize = mi.rtinfo.w;
    vec3  gridMin   = mi.rtinfo.xyz;
    vec3  gridSize  = mi.addinfo.yzw;
    vec3  gridMax   = gridMin + gridSize * voxelSize;
    float solidDepth = 0.0;
    for (int probe = 0; probe < 4; probe++) {
        vec3  pp  = rayOrigin + rayDir * voxelSize * float(probe);
        vec3  pUV = (pp - gridMin) / (voxelSize * gridSize);
        float occ = texture(sampler3D(texTexture, attachmentSampler), pUV).a;
        if (occ > 0.5) solidDepth += 1.0;
        else break;
    }
    rayOrigin += rayDir * voxelSize * (solidDepth + 1.0);
    vec3  invDir = 1.0 / (rayDir + vec3(1e-6));
    vec3  t0 = (gridMin - rayOrigin) * invDir;
    vec3  t1 = (gridMax - rayOrigin) * invDir;
    vec3  tMinV = min(t0, t1);
    vec3  tMaxV2 = max(t0, t1);
    float tEnter = max(max(tMinV.x, tMinV.y), tMinV.z);
    float tExit = min(min(tMaxV2.x, tMaxV2.y), tMaxV2.z);
    if (tEnter > tExit || tExit < 0.0) return false;
    float t        = max(tEnter, 0.0);
    vec3  localPos = (rayOrigin + rayDir * (t + 1e-4) - gridMin) / voxelSize;
    ivec3 voxel = clamp(ivec3(floor(localPos)), ivec3(0), ivec3(gridSize) - 1);
    ivec3 vstep = ivec3(sign(rayDir));
    vec3  tDelta = abs(voxelSize / rayDir);
    vec3  frac = localPos - floor(localPos);
    vec3 tMaxV;
    tMaxV.x = (rayDir.x > 0.0 ? 1.0 - frac.x : frac.x) * tDelta.x;
    tMaxV.y = (rayDir.y > 0.0 ? 1.0 - frac.y : frac.y) * tDelta.y;
    tMaxV.z = (rayDir.z > 0.0 ? 1.0 - frac.z : frac.z) * tDelta.z;
    int maxSteps = min(int(lightDist / voxelSize) + 2, MAX_STEPS);

    for (int i = 0; i < maxSteps; i++) {
        if (any(lessThan(voxel, ivec3(0))) || any(greaterThanEqual(voxel, ivec3(gridSize)))) return false;

        float currentT = min(tMaxV.x, min(tMaxV.y, tMaxV.z));
        if (currentT * voxelSize >= lightDist) return false;
        vec3  uvw = (vec3(voxel) + 0.5) / gridSize;
        float occ = texture(sampler3D(texTexture, attachmentSampler), uvw).a;
        if (occ > 0.5) return true;

        if (tMaxV.x < tMaxV.y && tMaxV.x < tMaxV.z) {
            tMaxV.x += tDelta.x; voxel.x += vstep.x;
        } else if (tMaxV.y < tMaxV.z) {
            tMaxV.y += tDelta.y; voxel.y += vstep.y;
        } else {
            tMaxV.z += tDelta.z; voxel.z += vstep.z;
        }
    }
    return false;
}

vec3 EvaluateDirectLighting(vec3 pos, vec3 N, vec3 V, vec3 albedo, float roughness, float metallic) {
    int   lightCount = clamp(int(mi.lightinfo.w), 0, 100);
    vec3  F0         = mix(vec3(0.04), albedo, metallic);
    vec3  total      = vec3(0.0);

    for (int li = 0; li < lightCount; li++) {
        vec3  lpos    = smi.lightpos[li].xyz;
        vec3  lcol    = smi.lightcol[li].xyz;
        float lintens = 1.0;
        vec3  toLight   = lpos - pos;
        float lightDist = length(toLight);
        vec3  L         = toLight / lightDist;
        float NdotL     = max(dot(N, L), 0.0);
        if (NdotL <= 0.0) continue;
        bool occluded = VoxelShadowRay(pos + N * mi.rtinfo.w, lpos);
        if (occluded) continue;
        vec3  H     = normalize(V + L);
        float NdotV = max(dot(N, V),    0.0001);
        float NdotH = max(dot(N, H),    0.0001);
        float VdotH = max(dot(V, H),    0.0001);
        vec3  F = FresnelSchlick(VdotH, F0);
        float D = DistributionGGX(N, H, roughness);
        float G = GeometrySmith(N, V, L, roughness);
        vec3  specular  = (D * G * F) / max(4.0 * NdotV * NdotL, 1e-5);
        vec3  kD        = (1.0 - F) * (1.0 - metallic);
        vec3  diffuse   = kD * albedo / PI;
        float atten     = lintens / (1.0 + lightDist * lightDist * 0.05);
        total += (diffuse + specular) * lcol * atten * NdotL;
    }

    return total;
}

vec3 voxelplusssrt() {
    vec2 uv = vec2(fuv.x, 1.0 - fuv.y);

    float d = SampleDepth(uv);
    if (d >= 0.9999) return SKY_COLOR;
    vec3 albedo = SampleAlbedo(uv);
    vec3 rma    = SampleRMA(uv);
    vec3 normal = SampleNormal(uv);
    vec3 wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);
    float roughness = rma.r;
    float metallic  = rma.g;
    vec3 camForward, camRight, camUp;
    float fovDeg = dmi.deffrot[0].w;
    float aspect = mi.resolutions.x / mi.resolutions.y;
    vec3 primaryRay = get_raydir(uv, fovDeg, camForward, camRight, camUp);
    vec3 V = -primaryRay;

    int spp = 1;

    vec3 result = vec3(0.0);

    for (int j = 0; j < spp; j++) {
        vec2 bnUV = mod(uv * 2.0 + float(mi.addinfo.x + j) * GOLDEN_RATIO, 1.0);
        vec4 bn   = texture(sampler2D(noiseTexture, attachmentSampler), bnUV);
        vec3 throughput = albedo;
        BRDFSample brdf = finalSample(V, normal, albedo, roughness, metallic, bn.xyz);
        vec3 rayDir     = brdf.direction;
        throughput     *= brdf.weight;
        vec3 direct = EvaluateDirectLighting(
            wrldpos + normal * 0.05, normal, V,
            albedo, roughness, metallic
        );
        result += direct;
        vec3  bouncePos    = wrldpos;
        vec3  bounceNormal = normal;
        vec3  bounceAlbedo = vec3(0.0);
        float bounceRough  = 0.85;
        float bounceMetal  = 0.0;
        bool  bounceHit    = false;
        vec2 nextUV = RayDirToUV(rayDir, camForward, camRight, camUp, aspect, fovDeg);
        bool onScreen = nextUV.x >= 0.0 && nextUV.x <= 1.0 && nextUV.y >= 0.0 && nextUV.y <= 1.0;
        bool awayFromCam = dot(rayDir, normal) > 0.0;
        if (onScreen && awayFromCam) {
            vec3 marchPos = wrldpos + normal * 0.05;
            bool ssHit    = false;
            for (int s = 0; s < 64; s++) {
                marchPos    += rayDir * 0.1;
                vec3 uvd     = WorldPosToUVDepth(marchPos, dmi.defferedMVP[0]);
                if (uvd.x < 0.0 || uvd.x > 1.0 || uvd.y < 0.0 || uvd.y > 1.0) break;
                float sd    = SampleDepth(uvd.xy);
                float delta = uvd.z - sd;
                if (delta > 0.0 && delta < SS_THICKNESS) {
                    bouncePos    = WorldPosFromDepth(sd, uvd.xy, dmi.defferedMVPInverse[0]);
                    bounceNormal = SampleNormal(uvd.xy);
                    bounceAlbedo = SampleAlbedo(uvd.xy);
                    vec3 brma    = SampleRMA(uvd.xy);
                    bounceRough  = brma.r;
                    bounceMetal  = brma.g;
                    bounceHit    = true;
                    ssHit        = true;
                    break;
                }
            }
            if (!ssHit) {
                vec3 voxNormal;
                vec3 voxOrigin = wrldpos + normal * mi.rtinfo.w * 1.5;
                vec4 voxHit    = voxelRaycast(voxOrigin, rayDir, voxNormal);
                if (voxHit.a > 0.5) {
                    bouncePos    = voxOrigin;
                    bounceNormal = voxNormal;
                    bounceAlbedo = voxHit.rgb;
                    bounceHit    = true;
                }
            }
        } else {
            vec3 voxNormal;
            vec3 voxOrigin = wrldpos + normal * mi.rtinfo.w * 1.5;
            vec4 voxHit    = voxelRaycast(voxOrigin, rayDir, voxNormal);
            if (voxHit.a > 0.5) {
                bouncePos    = voxOrigin;
                bounceNormal = voxNormal;
                bounceAlbedo = voxHit.rgb;
                bounceHit    = true;
            }
        }
        if (!bounceHit) {
            result += throughput * SKY_COLOR;
            continue;
        }
        vec3 bounceV      = -rayDir;
        vec3 bounceDirect = EvaluateDirectLighting(
            bouncePos + bounceNormal * 0.05, bounceNormal, bounceV,
            bounceAlbedo, bounceRough, bounceMetal
        );
        result += throughput * bounceDirect;
        vec2 bnUV2 = mod(bnUV + 0.5, 1.0);
        vec4 bn2   = texture(sampler2D(noiseTexture, attachmentSampler), bnUV2);
        BRDFSample brdf2  = finalSample(
            bounceV, bounceNormal, bounceAlbedo,
            bounceRough, bounceMetal, bn2.xyz
        );
        vec3 rayDir2   = brdf2.direction;
        throughput    *= brdf2.weight;
        vec3 voxOrigin2 = bouncePos + bounceNormal * mi.rtinfo.w * 1.5;
        vec3 voxNormal2;
        vec4 voxHit2    = voxelRaycast(voxOrigin2, rayDir2, voxNormal2);
        if (voxHit2.a > 0.5) {
            vec3 bounceV2 = -rayDir2;
            vec3 bounceDirect2 = EvaluateDirectLighting(voxOrigin2 + voxNormal2 * 0.05, voxNormal2, bounceV2, voxHit2.rgb, 0.85, 0.0);
            result += throughput * bounceDirect2;
        } else {
            result += throughput * SKY_COLOR;
        }
    }
    result /= float(spp);
    result = min(result, vec3(FIREFLY_CAP));
    return result;
}

void main() {
    outColor = vec4(voxelplusssrt(), 1.0);
}