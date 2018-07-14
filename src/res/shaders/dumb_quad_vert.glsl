#version 330 core
layout (location = 0) in vec3 position;

uniform mat4 model;
uniform mat4 projection_view;

out vec3 world_position;
// out vec3 rgb;

void main()
{
	// world_position = position;
	vec4 world_space_position = model * vec4(position, 1.0);
	world_position = world_space_position.xyz;

    gl_Position = projection_view * world_space_position;
    // gl_Position = vec4(world_position, 1.0);
}
