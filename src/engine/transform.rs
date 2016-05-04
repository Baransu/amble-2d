extern crate math;

use self::math::vec3::Vec3;
use self::math::vec2::Vec2;
use self::math::mat4::Mat4;

#[derive(Copy, Clone)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Vec3, // euler angles
    pub scale: Vec3,
}

impl Transform3D {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Transform3D {
        Transform3D { position: position, rotation: rotation, scale: scale }
    }

    fn calculate_model_matrix(&self) -> Mat4 {
        let scale_matrix = Mat4::from_scale(self.scale);
        let rotation_matrix = Mat4::from_rotation(self.rotation);
        let translation_matrix = Mat4::from_translation(self.position);

        scale_matrix * rotation_matrix * translation_matrix
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        self.calculate_model_matrix()
    }

}

#[derive(Copy, Clone)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2D {
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Transform2D {
        Transform2D { position: position, rotation: rotation, scale: scale }
    }

    fn calculate_model_matrix(&self) -> Mat4 {
        let scale_matrix = Mat4::from_scale(self.scale.to_vec3(1.0));
        let rotation_matrix = Mat4::from_rotation(Vec3::new(0.0, 0.0, self.rotation));
        let translation_matrix = Mat4::from_translation(self.position.to_vec3(-1.0));

        scale_matrix * rotation_matrix * translation_matrix
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        self.calculate_model_matrix()
    }

}
