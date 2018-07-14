#version 330 core

layout (location = 0) out vec3 position;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec4 color; // rgb = diffuse, a = specular

in vec3 world_position;
// in vec3 rgb;

void main()
{
    position = world_position;
    normal = vec3(0.0, 0.0, 1.0);
    color = vec4(0.5, 1.0, 0.5, 1.0);
    // color = vec4(rgb, 1.0);
}
