use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(source: Vector, target: Vector) -> Ray {
        let direction = (target - source).get_normal_vec();
        return Ray {
            direction,
            origin: source,
        };
    }

    pub fn get_reflected_ray(incident_ray: &Ray, normal: Vector, collision_point: Vector) -> Ray {
        let planar_component_direction = normal
            .cross_product(incident_ray.direction.cross_product(normal))
            .get_normal_vec();
        let planar_scalar_multiplier = incident_ray
            .direction
            .dot_product(planar_component_direction);
        let planar_component = planar_component_direction.mul_with_scalar(planar_scalar_multiplier);
        let normal_component = normal.mul_with_scalar(
            incident_ray
                .direction
                .mul_with_scalar(-1.0)
                .dot_product(normal),
        );
        return Ray {
            direction: planar_component + normal_component,
            origin: collision_point,
        };
    }
}
