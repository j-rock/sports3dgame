use app::StatusOr;
use file;
use geometry::Transform;
use gl::{
    self,
    types::*,
};
use glm;
use shader::ShaderProgram;
use std;

pub struct DumbQuad {
    shader: ShaderProgram,
    transform: Transform,
    quad_vao: GLuint,
    quad_vbo: GLuint,
}

impl DumbQuad {
    pub fn new() -> StatusOr<DumbQuad> {
        let vert_path = file::util::resource_path("shaders", "dumb_quad_vert.glsl");
        let frag_path = file::util::resource_path("shaders", "dumb_quad_frag.glsl");
        let shader = ShaderProgram::from_short_pipeline(vert_path.as_str(), frag_path.as_str())?;
        let mut dumb_quad = DumbQuad {
            shader,
            transform: Transform::new(),
            quad_vao: 0,
            quad_vbo: 0,
        };

        // Prepare lighting pass quad.
        let vertices: [f32; 12] = [
            // positions
            0.0, 0.8, 0.0,
            0.0, 0.0, 0.0,
            0.5, 1.0, 0.0,
            1.0, 0.0, 0.0,
        ];
        unsafe {
            gl::GenVertexArrays(1, &mut dumb_quad.quad_vao);
            gl::GenBuffers(1, &mut dumb_quad.quad_vbo);
            gl::BindVertexArray(dumb_quad.quad_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, dumb_quad.quad_vbo);
            let float_size = std::mem::size_of::<f32>() as isize;
            let vertex_array_byte_size = vertices.len() as isize * float_size;
            gl::BufferData(gl::ARRAY_BUFFER, vertex_array_byte_size, vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            // Vertex positions goes into attrib array = 0
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * float_size as i32, std::ptr::null());
        }
        Ok(dumb_quad)
    }

    pub fn draw(&self, projection_view: &glm::Mat4) {
        self.shader.activate();
        let model = self.transform.model();
        self.shader.set_mat4("model", &model);
        self.shader.set_mat4("projection_view", projection_view);
        unsafe {
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}