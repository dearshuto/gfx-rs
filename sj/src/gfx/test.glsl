#version 450

layout(std430, binding = 0) buffer Buffer
{
	uint u_Buffer[];
};

void main()
{
	u_Buffer[gl_GlobalInvocationID.x] = gl_GlobalInvocationID.x;
}
