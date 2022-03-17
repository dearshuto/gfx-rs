#version 450

layout (location = 0) out vec4 o_Color;

layout (location = 0) in vec3 v_Normal;

void main()
{
  float difuse = max(0.0, dot(v_Normal, vec3(-0.7, 0.0, -0.7)));
	o_Color = vec4(vec3(difuse), 1.0);
}
