use app::StatusOr;
use dimensions::time;
use file;
use gl::{
    self,
    types::*
};
use glm;
use nalgebra;
use ncollide;
use nphysics3d;
use shader::ShaderProgram;
use std::{
    self,
    sync::Arc,
};

const HEXAGON_SCALE: f32 = 20.0;

pub struct HexGrid {
    shader: ShaderProgram,
    vao: GLuint,
    positions_vbo: GLuint,
    positions: Vec<glm::Vec2>,
    heights_vbo: GLuint,
    heights: Vec<f32>,
}

impl HexGrid {
    pub fn new(world: &mut nphysics3d::world::World<f32>) -> StatusOr<HexGrid> {
        let shader = {
            let vert_path = file::util::resource_path("shaders", "hex_grid_vert.glsl");
            let geo_path = file::util::resource_path("shaders", "hex_grid_geo.glsl");
            let frag_path = file::util::resource_path("shaders", "hex_grid_frag.glsl");
            ShaderProgram::from_long_pipeline(vert_path.as_str(), geo_path.as_str(), frag_path.as_str())?
        };

        let dirs = vec!(glm::vec2(-1.0,1.0), glm::vec2(0.0,1.0), glm::vec2(1.0,0.0),
                        glm::vec2(1.0,-1.0), glm::vec2(0.0,-1.0), glm::vec2(-1.0,0.0));
        let radius = 2;
        let mut cursor = dirs[5] * (radius as f32);

        let mut positions = Vec::with_capacity(radius as usize * dirs.len());
        for dir in dirs.iter() {
            for _ in 0..radius {
                positions.push(cursor);
                cursor = cursor + *dir;
            }
        }

        let mut heights = Vec::with_capacity(positions.len());
        for position in positions.iter() {
            let combo = (position.x + position.y) as u32;
            heights.push(if combo % 2 == 0 {20.0} else {10.0});
        }

        let mut hex_grid = HexGrid {
            shader,
            vao: 0,
            positions_vbo: 0,
            positions,
            heights_vbo: 0,
            heights,
        };
        hex_grid.gl_init();
        hex_grid.update_physics(world);
        Ok(hex_grid)
    }

    fn gl_init(&mut self) {
        unsafe {
            // Setup vao.
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            // Attribute 0 --> positions.
            gl::GenBuffers(1, &mut self.positions_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.positions_vbo);
            let vec2_size = std::mem::size_of::<glm::Vec2>() as isize;
            let positions_size = self.positions.len() as isize * vec2_size;
            gl::BufferData(gl::ARRAY_BUFFER, positions_size, self.positions.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, vec2_size as i32, std::ptr::null());
            gl::VertexAttribDivisor(0, 1);

            // Attribute 1 --> heights.
            gl::GenBuffers(1, &mut self.heights_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.heights_vbo);
            let f32_size = std::mem::size_of::<f32>() as isize;
            let heights_size = self.heights.len() as isize * f32_size;
            gl::BufferData(gl::ARRAY_BUFFER, heights_size, self.heights.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, f32_size as i32, std::ptr::null());
            gl::VertexAttribDivisor(1, 1);

            // Cleanup
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn axial_to_cartesian() -> glm::Mat2 {
        let r3 = 3.0_f32.sqrt();
        glm::Mat2::new(glm::vec2(3.0, r3), glm::vec2(3.0, -r3)) * (HEXAGON_SCALE / 2.0)
    }
    pub fn draw(&self, projection_view: &glm::Mat4) {
        self.shader.activate();

        let axial_to_cartesian_mat = Self::axial_to_cartesian();
        self.shader.set_f32("hexagon_scale", HEXAGON_SCALE);
        self.shader.set_mat2("axial_to_cartesian", &axial_to_cartesian_mat);
        self.shader.set_mat4("projection_view", projection_view);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::POINTS, 0, 36, self.positions.len() as GLsizei);
            gl::BindVertexArray(0);
        }
    }

    pub fn update(&mut self, _dt: time::DeltaTime) {
       // No-op
    }

    fn generate_world_vertices_for_axial_coord(axial_to_cartesian_mat: &glm::Mat2, axial: glm::Vec2, height: f32) -> Vec<nalgebra::Point3<f32>> {
        // Algorithm ripped from hex_grid_geo.glsl
        let right_shift = glm::vec2(HEXAGON_SCALE, 0.0);
        let bl = *axial_to_cartesian_mat * axial;
        let tl = *axial_to_cartesian_mat * (axial + glm::vec2(1.0,-1.0));
        let hex_vertices_2d = vec!(
            bl,
            bl + right_shift, // br
            *axial_to_cartesian_mat * (axial + glm::vec2(0.0,-1.0)) + right_shift, // ml
            *axial_to_cartesian_mat * (axial + glm::vec2(1.0,0.0)), // mr
            tl,
            tl + right_shift); // tr
        hex_vertices_2d.into_iter().map(|vec2| nalgebra::Point3::new(vec2.x, height, -vec2.y)).collect()
    }

    fn update_physics(&mut self, world: &mut nphysics3d::world::World<f32>) {
        let axial_to_cart = Self::axial_to_cartesian();
        for i in 0..self.positions.len() {
            let position = self.positions[i];
            let height = self.heights[i];
            // hexagon = bl, br, ml, mr, tl, tr
            let vertices = Self::generate_world_vertices_for_axial_coord(&axial_to_cart, position, height);
            let indices = vec!(
                nalgebra::Point3::new(4, 2, 0), // tl -> ml -> bl
                nalgebra::Point3::new(4, 0, 3), // tl -> bl -> mr
                nalgebra::Point3::new(4, 3, 5), // tl -> mr -> tr
                nalgebra::Point3::new(0, 1, 3), // bl -> br -> mr
            );
            let mesh = ncollide::shape::TriMesh::new(Arc::new(vertices), Arc::new(indices), None, None);

            let rigid_body = nphysics3d::object::RigidBody::new_static(mesh, 1.0, 0.0);
            world.add_rigid_body(rigid_body);
        }
    }
}

impl Drop for HexGrid {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.positions_vbo != 0 {
                gl::DeleteBuffers(1, &self.positions_vbo);
            }
            if self.heights_vbo != 0 {
                gl::DeleteBuffers(1, &self.heights_vbo);
            }
        }
    }
}