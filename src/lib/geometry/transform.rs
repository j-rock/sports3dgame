use glm::{
    self,
    Mat4,
    Vec3
};

pub struct Transform {
    pub roll: f32,
    pub yaw: f32,
    pub position: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            roll: 0.0,
            yaw: 0.0,
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn model(&self) -> Mat4 {
        // Model*v = (T * R.yaw * R.roll * S)*v
        let model = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let model = glm::ext::translate(&model, self.position);
        let model = glm::ext::rotate(&model, self.yaw, glm::vec3(0.0, 1.0, 0.0));
        let model = glm::ext::rotate(&model, self.roll, glm::vec3(0.0, 0.0, 1.0));
        glm::ext::scale(&model, self.scale)
    }
}
