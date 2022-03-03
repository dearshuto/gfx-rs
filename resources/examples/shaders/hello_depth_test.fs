#version 450

layout(location = 0) out vec4 o_Color;

layout(location = 0) in vec3 v_Color;

void main()
{
    o_Color = vec4(v_Color.x, v_Color.y, v_Color.z, 1.0);
}