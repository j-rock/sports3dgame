#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_OUT {
	vec3 world_space_position;
} gs_in[];

out GS_OUT {
	vec3 normal;
	vec3 world_space_position;
} gs_out;

vec3 GetNormal() {
   vec3 a = gs_in[1].world_space_position - gs_in[0].world_space_position;
   vec3 b = gs_in[2].world_space_position - gs_in[0].world_space_position;
   return normalize(cross(a, b));
}

void main() {
	gs_out.normal = GetNormal();

    gl_Position = gl_in[0].gl_Position;
	gs_out.world_space_position = gs_in[0].world_space_position;
    EmitVertex();
    gl_Position = gl_in[1].gl_Position;
	gs_out.world_space_position = gs_in[1].world_space_position;
    EmitVertex();
    gl_Position = gl_in[2].gl_Position;
	gs_out.world_space_position = gs_in[2].world_space_position;
    EmitVertex();

    EndPrimitive();
}
