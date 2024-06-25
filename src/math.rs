use std::f32::consts::PI;

use glam::{Quat, Vec3};

pub fn create_model_matrix(position: (f32, f32), size: (f32, f32), rotation_degrees: f32) -> glam::f32::Mat4
{
    glam::f32::Mat4::from_scale_rotation_translation
    (
        Vec3::new(size.0, size.1, 1.0),
        Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), rotation_degrees / 180.0 * PI),
        Vec3::new(position.0, position.1, 0.0 ) 
    )
}

pub fn create_projection_matrix() -> glam::f32::Mat4
{
    glam::f32::Mat4::orthographic_rh_gl(0.0, 800.0, 0.0, 600.0, -1.0, 1.0)
}