#version 330 core
layout (location = 0) in vec2 axial_coord;
layout (location = 1) in float height;

out VS_OUT {
	float height;
} vs_out;

void main() {
	vs_out.height = height;
    gl_Position = vec4(axial_coord, 0.0, 1.0);
} 