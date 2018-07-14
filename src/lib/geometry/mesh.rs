use app::StatusOr;
use file;
use gl::{
    self,
    types::*,
};
use glm::{
    self,
    Vec3
};
use std::{
    io::Write,
    self,
};

fn vertex_index_parse(s: &str) -> StatusOr<VertexIndex> {
    s.parse::<VertexIndex>().map_err(|_err| format!("Couldn't parse VertexIndex in ({}).", s))
}

fn float_parse(s: &str) -> StatusOr<f32> {
   s.parse::<f32>().map_err(|_err| format!("Couldn't parse float in ({}).", s))
}

// An index into Mesh.vertices Vec.
pub type VertexIndex = GLuint;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<VertexIndex>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Mesh {
    pub fn new() -> Mesh {
        Self::from_geometry(vec!(), vec!())
    }

    pub fn from_geometry(vertices: Vec<Vec3>, faces: Vec<VertexIndex>) -> Mesh {
        Mesh {
            vertices,
            faces,
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    pub fn gl_init(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let vertex_array_byte_size = (self.vertices.len() * std::mem::size_of::<Vec3>()) as isize;
            let vertices_ptr = std::mem::transmute(self.vertices.as_ptr());
            gl::BufferData(gl::ARRAY_BUFFER, vertex_array_byte_size, vertices_ptr, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            let faces_array_byte_size = (self.faces.len() * std::mem::size_of::<VertexIndex>()) as isize;
            let faces_ptr = std::mem::transmute(self.faces.as_ptr());
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, faces_array_byte_size, faces_ptr, gl::STATIC_DRAW);

            // At position 0, 3 floats will be passed (not normalized) w/ stride sizeof(Vec3) (no offset)
            let size_of_vec3_i32 = std::mem::size_of::<Vec3>() as i32;
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of_vec3_i32, std::ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.faces.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }

    pub fn write_to_obj(&self, out_path: &str) -> StatusOr<()> {
        let mut file_buffer = file::util::buffered_writer_for(out_path)
            .map_err(|_err| format!("Failed to open file for writing: {}", out_path))?;
        for v in self.vertices.iter() {
            writeln!(&mut file_buffer, "v {} {} {}", v.x, v.y, v.z)
                .map_err(|err| format!("Failed to write vertex to file: {}", err))?;
        }
        for face_arr in self.faces.chunks(3) {
            writeln!(&mut file_buffer, "f {} {} {}",
                     face_arr[0] + 1,
                     face_arr[1] + 1,
                     face_arr[2] + 1)
                .map_err(|err| format!("Failed to write vertex to file: {}", err))?;
        }
        Ok(())
    }

    pub fn load_obj(filepath: &'static str) -> StatusOr<Mesh> {
        let mut mesh = Mesh::new();
        let mut line_no = 1;
        let lines = file::util::lines(filepath)
            .map_err(|err| format!("Couldn't load OBJ file ({}): {}", filepath, err))?;
        for line in lines {
            let unwrapped_line = line.map_err(|err| format!("[Bad line in {}:{}] {}", filepath, line_no, err))?;
            let parts: Vec<&str> = unwrapped_line.split(' ').collect();
            if parts[0] == "v" {
                mesh.vertices.push(glm::vec3(float_parse(parts[1])?, float_parse(parts[2])?, float_parse(parts[3])?));
            } else if parts[0] == "f" {
                mesh.faces.push(vertex_index_parse(parts[1])?);
                mesh.faces.push(vertex_index_parse(parts[2])?);
                mesh.faces.push(vertex_index_parse(parts[3])?);
            } else {
                return Err(format!("[Bad line in {}:{}] {}", filepath, line_no, unwrapped_line));
            }
            line_no += 1;
        }
        Ok(mesh)
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.ebo != 0 {
                gl::DeleteBuffers(1, &self.ebo);
            }
        }
    }
}
