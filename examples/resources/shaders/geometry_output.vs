#version 450

layout(location = 0) out vec3 v_Normal;

layout(location = 0) in vec3 i_Position;
layout(location = 1) in vec3 i_Normal;

layout(binding = 0, set = 0) uniform View {
    mat4 u_ProjectionViewMatrix;
};

void main() {
    vec4 worldPosition = u_ProjectionViewMatrix * vec4(i_Position, 1.0);
    gl_Position = worldPosition;
    v_Normal = i_Normal;
}
