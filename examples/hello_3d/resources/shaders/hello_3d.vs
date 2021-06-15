#version 450

layout (location = 0) out vec3 v_VertexColor;

layout (location = 0) in vec3 i_Position;

layout (binding = 0) uniform ConstantBuffer {
	mat4 u_ProjectionViewMatrix;
};

void main()
{
	gl_Position = u_ProjectionViewMatrix * vec4(i_Position, 1.0);
	v_VertexColor = 0.5 * (i_Position + 1.0);
}
