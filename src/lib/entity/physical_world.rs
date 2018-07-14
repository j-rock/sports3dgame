use app::StatusOr;
use controls::KeyboardControls;
use dimensions::time::DeltaTime;
use entity::{
    Athlete,
    Ball,
    HexGrid,
};
use glm;
use nalgebra;
use nphysics3d;

pub struct PhysicalWorld {
    athlete: Athlete,
    ball: Ball,
    hex_grid: HexGrid,
    world: nphysics3d::world::World<f32>,
}

impl PhysicalWorld {
    pub fn new() -> StatusOr<PhysicalWorld> {
        let mut world = nphysics3d::world::World::new();
        world.set_gravity(nalgebra::Vector3::new(0.0, -50.00, 0.0));

        let hex_grid = HexGrid::new(&mut world)?;

        Ok(PhysicalWorld {
            athlete: Athlete::new()?,
            ball: Ball::new()?,
            hex_grid,
            world,
        })
    }

    pub fn update(&mut self, keyboard: &KeyboardControls, dt: DeltaTime) {
        // Pre-physics step
        self.athlete.update(dt, keyboard, &mut self.world);
        self.ball.update(dt, &mut self.world);
        self.hex_grid.update(dt);

        // Apply physics
        self.world.step(dt.as_f32_seconds());

        // Post-physics step
        self.athlete.apply_physics();
        self.ball.apply_physics();
    }

    pub fn draw(&self, projection_view: &glm::Mat4) {
        self.athlete.draw(projection_view);
        self.ball.draw(projection_view);
        self.hex_grid.draw(projection_view);
    }
}
