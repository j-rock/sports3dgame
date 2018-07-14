use glm;
use nalgebra;
use ncollide;

pub struct MeshCuboid {
   x_min: f32,
   x_max: f32,
   y_min: f32,
   y_max: f32,
}

impl MeshCuboid {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> MeshCuboid {
        MeshCuboid {
            x_min, x_max, y_min, y_max
        }
    }
    pub fn cuboid(&self, scale: glm::Vec3) -> ncollide::shape::Cuboid<nalgebra::Vector3<f32>> {
        let half_extents = self.half_extents();
        let scaled = nalgebra::Vector3::new(scale.x * half_extents.x, scale.y * half_extents.y, scale.z * half_extents.z);
        ncollide::shape::Cuboid::new(scaled)
    }

    pub fn object_center(&self) -> glm::Vec3 {
        let half_extents = self.half_extents();
        glm::vec3(self.x_min + half_extents.x, self.y_min + half_extents.y, -0.5)
    }

    fn half_extents(&self) -> nalgebra::Vector3<f32> {
        let half_x = (self.x_max - self.x_min) / 2.0;
        let half_y = (self.y_max - self.y_min) / 2.0;
        nalgebra::Vector3::new(half_x, half_y, 0.5)
    }
}
