#version 450

layout(location = 0) out vec2 v_Uv;

void main() {
    float x = -1 + 2.0 * (gl_VertexIndex % 2) / 2;
    float y = -1 + 2.0 * gl_VertexIndex / 2;
    vec2 position = vec2(x, y);

    gl_Position = vec4(position, 0.0, 1.0);
    v_Uv = 0.5 * (position + 1.0);
}
