#version 450

layout(location=0) in vec3 a_position;

layout(set=1, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}