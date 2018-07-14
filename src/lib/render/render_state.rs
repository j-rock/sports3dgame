use app::StatusOr;
use controls::KeyboardControls;
use dimensions::time::DeltaTime;
use entity::PhysicalWorld;
use gl;
use render::{
    Camera,
    GBuffer,
};
use sdl2::render::WindowCanvas;

pub struct RenderState {
    camera: Camera,
    g_buffer: GBuffer,
    physical_world: PhysicalWorld,
    // std::unique_ptr<frame_rate> render_frames;
}

impl RenderState {
    pub fn new(width: i32, height: i32) -> StatusOr<RenderState> {
        unsafe { gl::Enable(gl::DEPTH_TEST); }
        // render_frames.reset(new frame_rate);
        let mut render = RenderState {
            camera: Camera::new(),
            g_buffer: GBuffer::new()?,
            physical_world: PhysicalWorld::new()?,
        };
        render.resize(width, height)?;
        Ok(render)
    }

    pub fn update(&mut self, keyboard: &KeyboardControls, dt: DeltaTime) {
        self.camera.update(keyboard, dt);
        self.physical_world.update(keyboard, dt);
        // render_frames->Update(dt);
    }

    pub fn resize(&mut self, width: i32, height: i32) -> StatusOr<()> {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
        self.g_buffer.gl_init(width, height)?;
        Ok(())
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        unsafe {
            gl::ClearColor(0.0177, 0.0177, 0.0477, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // 1. Draw all geometry.
        self.g_buffer.geometry_pass(); {
            let projection_view = self.camera.projection(canvas) * self.camera.view();
            self.physical_world.draw(&projection_view);
        }

        // 2. Lighting pass
        self.g_buffer.lighting_pass();

        // 3. Non-geometric superimposed draw calls.
        // TODO: Copy depth buffer.
        // render_frames->Draw(window);

        canvas.present();
    }
}