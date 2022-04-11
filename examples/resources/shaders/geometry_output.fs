#version 450

layout(location = 0) out vec4 o_Albedo;
layout(location = 1) out vec4 o_Normal;

layout(location = 0) in vec3 v_Normal;

void main() {
    o_Albedo = vec4(1.0);
    o_Normal = vec4((v_Normal + 1.0) / 2.0, 1.0);
}
