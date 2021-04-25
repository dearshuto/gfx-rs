#version 450

layout (std140, binding = 0) uniform ValueBlock
{
	uint u_Value;
};

layout (std140, binding = 1) uniform DataBlock
{
	uint u_Data;
};

layout (std430, binding = 0) buffer Buffer
{
	uint u_Buffer[];
};

void main()
{
	
}
