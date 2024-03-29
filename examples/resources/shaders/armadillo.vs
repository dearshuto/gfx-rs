#version 450

layout (location = 0) out vec3 v_Normal;

layout (location = 0) in vec3 i_Position;
layout (location = 1) in vec3 i_Normal;

layout (binding = 0) uniform UniformBlock {
	mat4 u_ProjectionViewMatrix;
};

void main()
{
	gl_Position = u_ProjectionViewMatrix * vec4(i_Position, 1.0);
  v_Normal = i_Normal;
}
