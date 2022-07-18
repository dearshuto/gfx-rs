#version 410

layout(location = 0) out vec3 v_Color;

layout(location = 0) in vec2 i_Position;
layout(location = 1) in vec3 i_Color;

void main()
{
  v_Color = i_Color;
  gl_Position = vec4(i_Position, 0.0, 1.0);
}
