use app::{
    StatusOr,
    Ticker
};
use dimensions::{
    Pixels,
    time::{
        DeltaTime,
        Microseconds
    },
};
use file;
use geometry::Mesh;
use gl;
use glm;
use image::{
    png::ImageRect,
    Png,
    Texture,
};
use render::sprite_3d::reify_sprite_3d;
use shader::ShaderProgram;

pub struct Sprite3DSheetInfo<'a> {
    pub filepath: &'a str,
    // Expects spritesheets to be 1 row by N images
    pub segment_width: Pixels,
    pub time_per_segment: Microseconds,
    pub all_states: Vec<Vec<usize>>,
}

pub struct Sprite3D {
    segment_width: Pixels,
    meshes: Vec<Mesh>,
    current_mesh_ticker: Ticker,
    current_state: usize,
    all_states: Vec<Vec<usize>>,
    texture_3d: Texture,
    shader_3d: ShaderProgram,
    pub object_center: glm::Vec3,
}

impl Sprite3D {
    pub fn new(options: Sprite3DSheetInfo) -> StatusOr<Sprite3D> {
        let png = Png::from_file(options.filepath)?;
        let texture_3d = Texture::new(&png);
        let (sheet_width, sheet_height) = (texture_3d.width, texture_3d.height);
        let segment_width = options.segment_width;

        let num_segments = sheet_width / segment_width;
        let mut meshes = Vec::with_capacity(num_segments);
        for i in 0..num_segments {
            let segment_image = png.copy_sub_image(ImageRect::new(0, i * segment_width, segment_width, sheet_height));
            let mut mesh = reify_sprite_3d::from_image(segment_image);
            mesh.gl_init();
            meshes.push(mesh);
        }
        let current_mesh_ticker = Ticker::new(options.time_per_segment);

        let vert_path = file::util::resource_path("shaders", "sprite_3d_vert.glsl");
        let geo_path = file::util::resource_path("shaders", "sprite_3d_geo.glsl");
        let frag_path = file::util::resource_path("shaders", "sprite_3d_frag.glsl");
        let shader_3d = ShaderProgram::from_long_pipeline(vert_path.as_str(), geo_path.as_str(), frag_path.as_str())?;

        Ok(Sprite3D {
            segment_width,
            meshes,
            current_mesh_ticker,
            current_state: 0,
            all_states: options.all_states,
            texture_3d,
            shader_3d,
            object_center: glm::vec3(0.0, 0.0, 0.0),
        })
    }

    pub fn draw(&self, projection_view: &glm::Mat4, model: &glm::Mat4) {
        self.shader_3d.activate();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_3d.texture_id);
        }
        // let animation_index = self.current_mesh_ticker.get_tick() % self.meshes.len();
        let current_mesh_indexes = &self.all_states[self.current_state];
        let animation_index = self.current_mesh_ticker.get_tick() % current_mesh_indexes.len();
        let mesh_index = current_mesh_indexes[animation_index];
        let material_scale = glm::vec2(self.texture_3d.width as f32, self.texture_3d.height as f32);
        self.shader_3d.set_vec3("center", &self.object_center);
        self.shader_3d.set_vec2("scale", &material_scale);
        self.shader_3d.set_mat4("model", model);
        self.shader_3d.set_mat4("projection_view", projection_view);
        self.shader_3d.set_f32("window_x_offset", (mesh_index * self.segment_width) as f32);
        self.shader_3d.set_i32("material", 0);
        self.meshes[mesh_index].draw();
    }

    pub fn write_to_objs(&self, path_prefix: &str) -> StatusOr<()> {
        let num_meshes = self.meshes.len();
        for i in 0..num_meshes {
            let out_path = format!("{}{}_{}.obj", path_prefix, i, num_meshes);
            self.meshes[i].write_to_obj(out_path.as_str())?;
        }
        Ok(())
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.current_mesh_ticker.update(dt);
    }

    pub fn set_state_index(&mut self, state: usize) -> StatusOr<()> {
       if state >= self.all_states.len() {
           Err(format!("Only have {} states. Trying to set state to {}", self.all_states.len(), state))
       } else {
           self.current_state = state;
           self.current_mesh_ticker.clear();
           Ok(())
       }
    }
}