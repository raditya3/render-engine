use crate::{material::material::Material, utils::vector::Vector};

pub trait SceneObject {
    fn get_surface_distance_from_point(&self, point: Vector) -> f64;
    fn normal_at_point(&self, point: Vector) -> Vector;
    fn get_material(&self) -> &dyn Material;
    fn get_position(&self) -> Vector;
}
