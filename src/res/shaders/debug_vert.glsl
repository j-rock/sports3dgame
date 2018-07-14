#version 330 core
layout (location = 0) in vec3 position;

out VS_OUT {
	vec3 world_space_position;
} vs_out;

uniform mat4 projection_view;

void main() {
    gl_Position = projection_view * vec4(position, 1.0);
	vs_out.world_space_position = position;
}
