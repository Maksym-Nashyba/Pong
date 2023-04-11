use nalgebra_glm::{Mat4x4, Quat, quat_cast, Vec2, Vec3};

#[derive(Clone)]
pub struct Transform{
    position: Vec3,
    rotation: Quat,
    scale: Vec3
}

impl Transform{
    pub fn to_transformation_matrix(&self) -> Mat4x4 {
        let mut result = Mat4x4::new_nonuniform_scaling(&self.scale);
        result = quat_cast(&self.rotation) * result;
        result = Mat4x4::new_translation(&self.position) * result;
        return result;
    }

    pub fn identity() -> Transform {
        return Transform::new(Vec2::zeros(), 0.0, Vec2::new(1.0,1.0));
    }

    pub fn new(position:Vec2, rotation:f32, scale:Vec2) -> Self {
        let position_raw:Vec3 = Vec3::new(position.x, position.y, 0.0);
        let rotation_raw:Quat = Quat::new((rotation/2.0).cos(), 0.0, 0.0, (rotation/2.0).sin());
        let scale_raw:Vec3 = Vec3::new(scale.x, scale.y, 1.0);
        return Self::raw(position_raw, rotation_raw, scale_raw);
    }

    fn raw(position:Vec3, rotation:Quat, scale:Vec3) -> Self{
        return Self{
            position,
            rotation,
            scale
        }
    }
}