#version 460 core
layout (location=0) in vec3 position;
layout (location=1) in vec3 normal;
layout (location=2) in vec2 texCoord;

layout (location=0) out VSOUT
{
	vec3 worldPos;
    vec3 normal;
    vec2 texCoord;
} vs_out;

layout (std140, binding = 0) uniform UBOCamera
{
	mat4 projection;
	mat4 view;
    mat4 viewProj;
    vec3 camPos;
} uboCamera;

void main() {
	vs_out.worldPos = position;
    vs_out.normal = normal;
	vs_out.texCoord = texCoord;
	gl_Position = uboCamera.viewProj * vec4(position, 1.0);
}