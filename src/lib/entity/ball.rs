use app::StatusOr;
use dimensions::{
    Pixels,
    time
};
use file;
use geometry::{
    MeshCuboid,
    Transform
};
use glm;
use nalgebra;
use ncollide;
use nphysics3d::{
    self,
    volumetric::Volumetric,
};
use num_traits::identities::Zero;
use render::{
    Sprite3D,
    Sprite3DSheetInfo
};
use serde_json;

#[derive(Deserialize)]
pub struct BallConfig {
    pub sprite_segment_width: Pixels,
    pub sprite_segment_millis: i64,
    pub states: Vec<usize>,
}

impl BallConfig {
    pub fn read() -> StatusOr<BallConfig> {
        let path = file::util::resource_path("config", "ball.config");
        let data = file::util::slurp_file(path)
            .map_err(|err| format!("Couldn't slurp ball config: {}", err))?;
        serde_json::from_str(data.as_str())
            .map_err(|err| format!("Couldn't read ball config: {}", err))
    }
}

pub struct Ball {
    pub sprite: Sprite3D,
    transform: Transform,
    cuboid_bounds: MeshCuboid,
    rigid_body: Option<nphysics3d::object::RigidBodyHandle<f32>>,
}

impl Ball {
    pub fn new() -> StatusOr<Ball> {
        let config = BallConfig::read()?;
        let filepath = file::util::resource_path("images", "ball.png");
        let sheet_info = Sprite3DSheetInfo {
            filepath: filepath.as_str(),
            segment_width: config.sprite_segment_width,
            time_per_segment: time::milliseconds(config.sprite_segment_millis),
            all_states: vec![config.states.clone()],
        };
        let mut ball = Ball {
            sprite: Sprite3D::new(sheet_info)?,
            transform: Transform::new(),
            cuboid_bounds: MeshCuboid::new(1.001, 13.999, 1.001, 13.999),
            rigid_body: None,
        };
        ball.transform.scale = glm::vec3(0.5, 0.5, 0.5);
        ball.transform.position = glm::vec3(0.0, 50.0, 130.0);
        ball.sprite.object_center = ball.cuboid_bounds.object_center();
        Ok(ball)
    }

    pub fn draw(&self, projection_view: &glm::Mat4) {
        let model = self.transform.model();
        self.sprite.draw(projection_view, &model);
    }

    pub fn update(&mut self, dt: time::DeltaTime, world: &mut nphysics3d::world::World<f32>) {
        self.sprite.update(dt);
        self.update_physics(world);
    }

    fn update_physics(&mut self, world: &mut nphysics3d::world::World<f32>) {
        if self.rigid_body.is_none() {
            let cuboid = self.cuboid_bounds.cuboid(self.transform.scale);
            let (mass, center_of_mass, _angular_inertia) = cuboid.mass_properties(1.0);
            let zero_angular_inertia = nphysics3d::math::AngularInertia::<f32>::zero();
            let shape_handle = ncollide::shape::ShapeHandle::new(cuboid);
            let mut rigid_body = nphysics3d::object::RigidBody::new(shape_handle, Some((mass, center_of_mass, zero_angular_inertia)), 0.7, 0.0);
            let pos = self.transform.position;
            rigid_body.set_translation(nalgebra::Translation3::new(pos.x, pos.y, pos.z));
            self.rigid_body = Some(world.add_rigid_body(rigid_body));
        } else if let Some(ref rigid_body) = self.rigid_body.as_ref() {
            let mut rigid_body_mut = rigid_body.borrow_mut();
            let vertical_lin_vel = rigid_body_mut.lin_vel().y;
            rigid_body_mut.set_lin_vel(nalgebra::Vector3::new(0.0, vertical_lin_vel, 0.0));
        }
    }

    pub fn apply_physics(&mut self) {
        if let Some(ref rigid_body) = self.rigid_body.as_ref() {
            let translation = rigid_body.borrow().position().translation.vector;
            self.transform.position = glm::vec3(translation.x, translation.y, translation.z);
        }
    }
}
