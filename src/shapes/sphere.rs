use crate::{material::material::Material, utils::vector::Vector};

use super::scene_object::SceneObject;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl SceneObject for Sphere {
    fn get_surface_distance_from_point(&self, point: Vector) -> f64 {
        let distance_to_center = (point - self.center).magnitude;
        return distance_to_center - self.radius;
    }
    fn normal_at_point(&self, point: Vector) -> Vector {
        return (point - self.center).get_normal_vec();
    }

    fn get_material(&self) -> &dyn Material {
        return &*self.material;
    }

    fn get_position(&self) -> Vector {
        return self.center;
    }
}
