#version 330 core
layout (location = 0) in vec3 position;

out VS_OUT {
	vec2 texture_coords;
	vec3 world_space_position;
} vs_out;

uniform vec3 center;
uniform vec2 scale;
uniform mat4 model;
uniform mat4 projection_view;
uniform float window_x_offset;

void main() {
	vec4 world_space_position = model * vec4(position - center, 1.0);
    gl_Position = projection_view *  world_space_position;

	vec2 t = position.xy + vec2(window_x_offset, 0.0);
	t /= scale;
	t.y = 1.0 - t.y;

    vs_out.texture_coords = t;
	vs_out.world_space_position = vec3(world_space_position);
} 