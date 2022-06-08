#version 450

layout(location = 0) out vec2 v_Uv;

layout(location = 0) in vec2 i_Position;
layout(location = 1) in vec2 i_Uv;

void main()
{
  v_Uv = i_Uv;
  gl_Position = vec4(i_Position, 0.0, 1.0);
}
