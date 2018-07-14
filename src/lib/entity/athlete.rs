use app::StatusOr;
use controls::KeyboardControls;
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
    Sprite3DSheetInfo,
};
use sdl2::keyboard::Scancode;
use serde_json;

#[derive(Deserialize)]
pub struct AthleteConfig {
    pub sprite_segment_width: Pixels,
    pub sprite_segment_millis: i64,
    pub sideways_states: Vec<usize>,
    pub backwards_states: Vec<usize>,
}

impl AthleteConfig {
    pub fn read() -> StatusOr<AthleteConfig> {
        let path = file::util::resource_path("config", "athlete.config");
        let data = file::util::slurp_file(path)
            .map_err(|err| format!("Couldn't slurp athlete config: {}", err))?;
        serde_json::from_str(data.as_str())
            .map_err(|err| format!("Couldn't read athlete config: {}", err))
    }
}

pub struct Athlete {
    pub sprite: Sprite3D,
    pub transform: Transform,
    cuboid_bounds: MeshCuboid,
    rigid_body: Option<nphysics3d::object::RigidBodyHandle<f32>>,
}

impl Athlete {
   pub fn new() -> StatusOr<Athlete> {
       let filepath = file::util::resource_path("images", "player.png");
       let config = AthleteConfig::read()?;
       let sheet_info = Sprite3DSheetInfo {
           filepath: filepath.as_str(),
           segment_width: config.sprite_segment_width,
           time_per_segment: time::milliseconds(config.sprite_segment_millis),
           all_states: vec![config.sideways_states.clone(), config.backwards_states.clone()],
       };
       let mut athlete = Athlete {
           sprite: Sprite3D::new(sheet_info)?,
           transform: Transform::new(),
           cuboid_bounds: MeshCuboid::new(12.0, 20.998, 1.001, 25.998),
           rigid_body: None,
       };
       athlete.transform.scale = glm::vec3(0.5, 0.5, 5.0);
       athlete.transform.position = glm::vec3(0.0, 29.5, 130.0);
       athlete.sprite.object_center = athlete.cuboid_bounds.object_center();
       Ok(athlete)
   }

    pub fn draw(&self, projection_view: &glm::Mat4) {
        let model = self.transform.model();
        self.sprite.draw(projection_view, &model);
    }

    pub fn update(&mut self, dt: time::DeltaTime, keyboard: &KeyboardControls, world: &mut nphysics3d::world::World<f32>) {
        if keyboard.just_released(Scancode::Y) || keyboard.just_released(Scancode::H) {
            self.sprite.set_state_index(0).unwrap();
        }
        if keyboard.just_pressed(Scancode::Y) || keyboard.just_pressed(Scancode::H) {
            self.sprite.set_state_index(1).unwrap();
        }

        self.sprite.update(dt);
        self.update_physics(keyboard, world);
    }

    fn update_physics(&mut self, keyboard: &KeyboardControls, world: &mut nphysics3d::world::World<f32>) {
        if self.rigid_body.is_none() {
            let cuboid = self.cuboid_bounds.cuboid(self.transform.scale);
            let (mass, center_of_mass, _angular_inertia) = cuboid.mass_properties(1.0);
            let zero_angular_inertia = nphysics3d::math::AngularInertia::<f32>::zero();
            let shape_handle = ncollide::shape::ShapeHandle::new(cuboid);
            let mut rigid_body = nphysics3d::object::RigidBody::new(shape_handle, Some((mass, center_of_mass, zero_angular_inertia)), 0.9, 0.0);
            let pos = self.transform.position;
            rigid_body.set_translation(nalgebra::Translation3::new(pos.x, pos.y, pos.z));
            self.rigid_body = Some(world.add_rigid_body(rigid_body));
        } else if let Some(ref rigid_body) = self.rigid_body.as_ref() {
            let mut rigid_body_mut = rigid_body.borrow_mut();
            let vertical_lin_vel = rigid_body_mut.lin_vel().y;
            let mut horizontal_lin_vel = glm::vec2(0.0, 0.0);
            if keyboard.is_pressed(Scancode::J) {
                horizontal_lin_vel.x += 5.0;
            }
            if keyboard.is_pressed(Scancode::G) {
                horizontal_lin_vel.x -= 5.0;
            }
            if keyboard.is_pressed(Scancode::Y) {
                horizontal_lin_vel.y -= 5.0;
            }
            if keyboard.is_pressed(Scancode::H) {
                horizontal_lin_vel.y += 5.0;
            }
            rigid_body_mut.set_lin_vel(nalgebra::Vector3::new(horizontal_lin_vel.x, vertical_lin_vel, horizontal_lin_vel.y));
        }
    }

    pub fn apply_physics(&mut self) {
        if let Some(ref rigid_body) = self.rigid_body.as_ref() {
            let translation = rigid_body.borrow().position().translation.vector;
            self.transform.position = glm::vec3(translation.x, translation.y, translation.z);
        }
    }
}