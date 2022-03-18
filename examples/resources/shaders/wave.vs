#version 450

layout (location = 0) out vec3 v_Normal;

layout (location = 0) in vec3 i_Position;
layout (location = 1) in vec3 i_Normal;

layout (binding = 0) uniform UniformBlock {
	mat4 u_ProjectionViewMatrix;
  float u_Time;
  float _0;
  float _1;
  float _2;
};

void main()
{
  float distance = length(i_Position.xz) / 0.075;

  vec3 position = vec3(i_Position.x, 0.1 * sin(u_Time + distance), i_Position.z);
	gl_Position = u_ProjectionViewMatrix * vec4(position, 1.0);
  v_Normal = 0.0 * i_Normal + 5.0 * (0.1 + position);
}
