#version 450

layout (location = 0) out vec4 o_Color;

layout (location = 0) in vec3 v_VertexColor;

void main()
{
	o_Color = vec4(v_VertexColor, 1.0);
}
