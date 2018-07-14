#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 36) out;

in VS_OUT {
	float height;
} vs_in[];

out GS_OUT {
    vec3 world_space_position;
	vec3 color;
	vec3 normal;
} gs_out;

// Distance from hexagon center to corner.
uniform float hexagon_scale;
uniform mat2 axial_to_cartesian;
uniform mat4 projection_view;

// XY coordinate for the bottom-left point of the hexagon.
vec2 ToCart2d(in vec2 axial) {
	return axial_to_cartesian * axial;
}

vec4 ToCart3d(in vec2 cart2d) {
	return vec4(cart2d.x, vs_in[0].height, -cart2d.y, 1.0);
}

vec3 Normal(in vec4 v0, in vec4 v1, in vec4 v2) {
   vec3 a = (v1 - v0).xyz;
   vec3 b = (v2 - v0).xyz;
   return normalize(cross(a, b));
}

void EmitHexagon(in vec4 ml, in vec4 bl, in vec4 tl, in vec4 br, in vec4 tr, in vec4 mr, 
				 in vec4 ml_3d, in vec4 bl_3d, in vec4 tl_3d, in vec4 br_3d, in vec4 tr_3d, in vec4 mr_3d, in vec3 normal) {
	gl_Position = ml;
	gs_out.world_space_position = ml_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.6, 0.0, 0.0);
	EmitVertex();

	gl_Position = bl;
    gs_out.world_space_position = bl_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(1.0, 1.0, 0.0);
	EmitVertex();

	gl_Position = tl;
	gs_out.world_space_position = tl_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(1.0, 0.0, 0.0);
	EmitVertex();

	gl_Position = br;
    gs_out.world_space_position = br_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.0, 0.2, 0.6);
	EmitVertex();

	gl_Position = tr;
    gs_out.world_space_position = tr_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(1.0, 1.0, 1.0);
	EmitVertex();

	gl_Position = mr;
    gs_out.world_space_position = mr_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.5, 0.0, 1.0);
	EmitVertex();

	EndPrimitive();
}

void EmitQuad(in vec4 tl, in vec4 bl, in vec4 br, in vec4 tr,
			  in vec4 tl_3d, in vec4 bl_3d, in vec4 br_3d, in vec4 tr_3d, in vec3 normal) {
	gl_Position = tl;
	gs_out.world_space_position = tl_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.3, 0.0, 4.0);
	EmitVertex();

	gl_Position = bl;
    gs_out.world_space_position = bl_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.0, 0.8, 0.0);
	EmitVertex();

	gl_Position = tr;
    gs_out.world_space_position = tr_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(0.5, 0.5, 1.0);
	EmitVertex();

	gl_Position = br;
    gs_out.world_space_position = br_3d.xyz;
	gs_out.normal = normal;
	gs_out.color = vec3(1.0, 0.5, 0.0);
	EmitVertex();

	EndPrimitive();
}

//     (1,-1)
// (0,-1)  (1,0)
//     (0,0)
// (-1,0)  (0,1)
//     (-1,1)

void main() {
    vec2 axial_coord = gl_in[0].gl_Position.xy;

	// 2D space coordinates
	  vec2 right_shift = vec2(hexagon_scale, 0.0);
	  // Each hexagon owns its bottom corners
	  vec2 bl = ToCart2d(axial_coord);
      vec2 br = bl + right_shift;
	  // Middle-left comes from top-left neighbor
	  vec2 ml = ToCart2d(axial_coord + vec2(0,-1)) + right_shift;
	  // Middle-right comes from top-right neighbor
      vec2 mr = ToCart2d(axial_coord + vec2(1,0));
	  // Top corners come from top neighbor.
	  vec2 tl = ToCart2d(axial_coord + vec2(1,-1));
      vec2 tr = tl + right_shift;

    // 3D world coordinates
	//    /tl _ tr\
	//  ml \      mr
	//   |  bl _ br/
	//   |   |   | |
	//  l_ml |   | l_mr
	//   \l_bl l_br/

	vec4 bl_3d = ToCart3d(bl);
	vec4 br_3d = ToCart3d(br);
	vec4 ml_3d = ToCart3d(ml);
	vec4 mr_3d = ToCart3d(mr);
	vec4 tl_3d = ToCart3d(tl);
	vec4 tr_3d = ToCart3d(tr);

	vec4 l_ml_3d = vec4(ml_3d.x, 0.0, ml_3d.z, 1.0); // Zero out y-comp
    vec4 l_mr_3d = vec4(mr_3d.x, 0.0, mr_3d.z, 1.0);
	vec4 l_bl_3d = vec4(bl_3d.x, 0.0, bl_3d.z, 1.0);
	vec4 l_br_3d = vec4(br_3d.x, 0.0, br_3d.z, 1.0);
    vec4 l_tl_3d = vec4(tl_3d.x, 0.0, tl_3d.z, 1.0);
	vec4 l_tr_3d = vec4(tr_3d.x, 0.0, tr_3d.z, 1.0);

	// Normals
	vec3 top = Normal(ml_3d, bl_3d, tl_3d);
	vec3 front = Normal(bl_3d, l_bl_3d, l_br_3d);
	vec3 left = Normal(ml_3d, l_ml_3d, l_bl_3d);
	vec3 right = Normal(br_3d, l_br_3d, l_mr_3d);

    // View space coordinates
	vec4 bl_v   = projection_view * bl_3d;
	vec4 br_v   = projection_view * br_3d;
	vec4 ml_v   = projection_view * ml_3d;
	vec4 mr_v   = projection_view * mr_3d;
	vec4 tl_v   = projection_view * tl_3d;
	vec4 tr_v   = projection_view * tr_3d;
    vec4 l_ml_v = projection_view * l_ml_3d;
    vec4 l_mr_v = projection_view * l_mr_3d;
    vec4 l_bl_v = projection_view * l_bl_3d;
	vec4 l_br_v = projection_view * l_br_3d;
    vec4 l_tl_v = projection_view * l_tl_3d;
	vec4 l_tr_v = projection_view * l_tr_3d;
	
	// Top
	EmitHexagon(ml_v, bl_v, tl_v, br_v, tr_v, mr_v,
				ml_3d, bl_3d, tl_3d, br_3d, tr_3d, mr_3d, top);
	// Bottom
    EmitHexagon(l_ml_v, l_bl_v, l_tl_v, l_br_v, l_tr_v, l_mr_v,
				l_ml_3d, l_bl_3d, l_tl_3d, l_br_3d, l_tr_3d, l_mr_3d, -top);
	// Front left
	EmitQuad(ml_v, l_ml_v, l_bl_v, bl_v,
			 ml_3d, l_ml_3d, l_bl_3d, bl_3d, left);
	// Back left
	EmitQuad(tl_v, l_tl_v, l_ml_v, ml_v,
			 tl_3d, l_tl_3d, l_ml_3d, ml_3d, -right);
	// Front
	EmitQuad(bl_v, l_bl_v, l_br_v, br_v,
		     bl_3d, l_bl_3d, l_br_3d, br_3d, front);
	// Back
    EmitQuad(tr_v, l_tr_v, l_tl_v, tl_v,
			 tr_3d, l_tr_3d, l_tl_3d, tl_3d, -front);
	// Front right
	EmitQuad(br_v, l_br_v, l_mr_v, mr_v,
			 br_3d, l_br_3d, l_mr_3d, mr_3d, right);
	// Back right
	EmitQuad(mr_v, l_mr_v, l_tr_v, tr_v,
			 mr_3d, l_mr_3d, l_tr_3d, tr_3d, -left);
}  