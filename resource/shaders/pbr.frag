#version 460 core
layout (location=0) out vec4 FragColor;

layout (location=0) in VSOUT
{
	vec3 worldPos;
    vec3 normal;
    vec2 texCoord;
} fs_in;


layout (std140, binding = 0) uniform UBOCamera
{
	mat4 projection;
	mat4 view;
    mat4 viewProj;
    vec3 camPos;
} uboCamera;

layout (binding=0) uniform sampler2D baseColorTexture;
layout (binding=1) uniform sampler2D metallicRoughnessTexture;
layout (binding=2) uniform sampler2D emissiveTexture;
layout (binding=3) uniform sampler2D aoTexture;
layout (binding=4) uniform sampler2D normalTexture;

struct Material {
    int baseColor;
    int metallicRoughness;
    int emissive;
    int ao;
    int normal;
};

layout (std430, binding = 3) readonly buffer UBOMaterial {
	Material materials[];
};



const float M_PI = 3.1415926;

struct PBRInfo
{
	float NdotL;                  // cos angle between normal and light direction
	float NdotV;                  // cos angle between normal and view direction
	float NdotH;                  // cos angle between normal and half vector
	float LdotH;                  // cos angle between light direction and half vector
	float VdotH;                  // cos angle between view direction and half vector
	float perceptualRoughness;    // roughness value, as authored by the model creator (input to shader)
	float metalness;              // metallic value at the surface
	vec3 reflectance0;            // full reflectance color (normal incidence angle)
	vec3 reflectance90;           // reflectance color at grazing angle
	float alphaRoughness;         // roughness mapped to a more linear change in the roughness (proposed by [2])
	vec3 diffuseColor;            // color contribution from diffuse lighting
	vec3 specularColor;           // color contribution from specular lighting
};

vec3 diffuse(PBRInfo pbrInputs)
{
	return pbrInputs.diffuseColor / M_PI;
}

vec3 specularReflection(PBRInfo pbrInputs)
{
	return pbrInputs.reflectance0 + (pbrInputs.reflectance90 - pbrInputs.reflectance0) * pow(clamp(1.0 - pbrInputs.VdotH, 0.0, 1.0), 5.0);
}

float geometricOcclusion(PBRInfo pbrInputs)
{
	float NdotL = pbrInputs.NdotL;
	float NdotV = pbrInputs.NdotV;
	float r = pbrInputs.alphaRoughness;

	float attenuationL = 2.0 * NdotL / (NdotL + sqrt(r * r + (1.0 - r * r) * (NdotL * NdotL)));
	float attenuationV = 2.0 * NdotV / (NdotV + sqrt(r * r + (1.0 - r * r) * (NdotV * NdotV)));
	return attenuationL * attenuationV;
}

float microfacetDistribution(PBRInfo pbrInputs)
{
	float roughnessSq = pbrInputs.alphaRoughness * pbrInputs.alphaRoughness;
	float f = (pbrInputs.NdotH * roughnessSq - pbrInputs.NdotH) * pbrInputs.NdotH + 1.0;
	return roughnessSq / (M_PI * f * f);
}


void main() {
	vec3 diffuseColor			= vec3(0.0);
	vec3 specularColor			= vec3(0.0);
	vec4 baseColor				= vec4(0.0, 0.0, 0.0, 1.0);
	vec3 f0						= vec3(0.04);
	float perceptualRoughness;
	float metallic;
    Material material = materials[0];
	
	vec4 mrSample = texture(metallicRoughnessTexture, fs_in.texCoord);
    perceptualRoughness = mrSample.g;
    metallic = mrSample.b;
    baseColor = texture(baseColorTexture, fs_in.texCoord);

    diffuseColor = baseColor.rgb * (vec3(1.0) - f0) * (1.0 - metallic);
    specularColor = mix(f0, baseColor.rgb, metallic);
    float alphaRoughness = perceptualRoughness * perceptualRoughness;
    float reflectance = max(max(specularColor.r, specularColor.g), specularColor.b);
    vec3 specularEnvironmentR0 = specularColor.rgb;
	vec3 specularEnvironmentR90 = vec3(clamp(reflectance * 50.0, 0.0, 1.0));
    vec3 normal = normalize(fs_in.normal);
    vec3 view = normalize(uboCamera.camPos - fs_in.worldPos);
    vec3 light = normalize(vec3(0, 0, 1));
    vec3 h = normalize(light + view);

	float NdotL = clamp(dot(normal, light),		0.001, 1.0);
	float NdotV = clamp(abs(dot(normal, view)), 0.001, 1.0);
	float NdotH = clamp(dot(normal, h),			  0.0, 1.0);
	float LdotH = clamp(dot(light, h),			  0.0, 1.0);
	float VdotH = clamp(dot(view, h),			  0.0, 1.0);

    PBRInfo pbr = PBRInfo(NdotL, NdotV, NdotH, LdotH, VdotH, perceptualRoughness,
						  metallic, specularEnvironmentR0, specularEnvironmentR90,
						  alphaRoughness, diffuseColor, specularColor);

    vec3 F = specularReflection(pbr);
	float G = geometricOcclusion(pbr);
	float D = microfacetDistribution(pbr);

    const vec3 kLightColor = vec3(4.0);

    vec3 diffuseContrib = (1.0 - F) * diffuse(pbr);
    vec3 specContrib = F * G * D / (4.0 * NdotL * NdotV);
    vec3 color = NdotL * kLightColor * (diffuseContrib + specContrib);

    float ao = texture(aoTexture, fs_in.texCoord).r;
    color = color * ao;
    vec3 emissive = texture(emissiveTexture, fs_in.texCoord).rgb;
    color += emissive;


	FragColor = vec4(color, 1.0);
}