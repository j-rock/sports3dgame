#version 330 core

layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in GS_OUT {
	vec2 texture_coords;
	vec3 normal;
	vec3 world_space_position;
} fs_in;

uniform sampler2D material;

void main() {
	position = fs_in.world_space_position;
	normal = fs_in.normal;
	color.rgb = vec3(texture(material, fs_in.texture_coords));
	color.a = 0.0;
}