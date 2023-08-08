use crate::camera::Camera;
use crate::light_source::LightSource;
use crate::shapes::scene_object::SceneObject;
use crate::utils::color::Color;
use crate::utils::ray::Ray;
use crate::utils::vector::Vector;

pub struct Engine {
    pub scene_objects: Vec<Box<dyn SceneObject>>,
    pub camera: Camera,
    pub light_source: LightSource,
}

impl Engine {
    pub fn generate_image(&self) -> Vec<Vec<(u8, u8, u8)>> {
        let screen_width = self.camera.screen_width as usize;
        let screen_height = self.camera.screen_height as usize;
        let mut image: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); screen_width]; screen_height];

        for i in 0..screen_height {
            for j in 0..screen_width {
                let ray_cast_from_camera = self.camera.get_ray_from_image_point(i as u32, j as u32);
                let color_of_ray = self.get_color_of_ray(
                    ray_cast_from_camera,
                    &self.scene_objects.iter().map(|x| x).collect(),
                    3,
                );
                image[j][i].0 = color_of_ray.r;
                image[j][i].1 = color_of_ray.g;
                image[j][i].2 = color_of_ray.b;
            }
        }

        return image;
    }
    // Test if any object is blocking line of sight at point
    fn is_direct_exposed_to_light_source(
        &self,
        point: Vector,
        scene_objects: Vec<&Box<dyn SceneObject>>,
    ) -> bool {
        let ray = Ray::new(point, self.light_source.position);
        let (nearest_scene_object, _light_particle_pos) =
            Self::get_intersect_object_of_ray(ray, scene_objects);

        return nearest_scene_object.is_none();
    }

    // Use ray marching to find object colliding with ray
    fn get_intersect_object_of_ray(
        ray: Ray,
        scene_objects: Vec<&Box<dyn SceneObject>>,
    ) -> (Option<&Box<dyn SceneObject>>, Vector) {
        const COLLISION_THRESHOLD: f64 = 0.02;
        const UPPER_DISTANCE_LIMIT: f64 = 2000.0;
        let mut nearest_scene_object: Option<&Box<dyn SceneObject>> = None;
        let mut light_particle_pos: Vector = ray.origin;
        let mut ray_travel_dist: f64 = 0.0;
        loop {
            let mut distance_to_nearest_obj = 1000.0;

            for scene_object in scene_objects.iter() {
                let distance = scene_object.get_surface_distance_from_point(light_particle_pos);
                if distance < distance_to_nearest_obj {
                    distance_to_nearest_obj = distance;
                    nearest_scene_object = Some(scene_object);
                }
            }

            if distance_to_nearest_obj <= COLLISION_THRESHOLD {
                break;
            }
            ray_travel_dist += distance_to_nearest_obj;
            if ray_travel_dist >= UPPER_DISTANCE_LIMIT {
                nearest_scene_object = None;
                break;
            }
            light_particle_pos =
                light_particle_pos + ray.direction.mul_with_scalar(distance_to_nearest_obj);
        }

        return (nearest_scene_object, light_particle_pos);
    }

    fn get_color_of_ray(
        &self,
        ray: Ray,
        scene_objects: &Vec<&Box<dyn SceneObject>>,
        // Number of light bounces to calculate
        depth: u8,
    ) -> Color {
        let (optional_nearest_scene_object, light_particle_pos) = Self::get_intersect_object_of_ray(
            ray.clone(),
            scene_objects.iter().map(|x| *x).collect(),
        );

        if optional_nearest_scene_object.is_none() {
            return Color::new(0, 0, 0);
        }
        let nearest_scene_object = optional_nearest_scene_object.unwrap();
        let surface_normal = nearest_scene_object.normal_at_point(light_particle_pos);

        let ray_from_light_source = Ray::new(self.light_source.position, light_particle_pos);

        let collision_material = nearest_scene_object.get_material();
        let direction_toward_light_source = ray_from_light_source.direction.mul_with_scalar(-1.0);

        let half_vector =
            (ray.direction.mul_with_scalar(-1.0) + direction_toward_light_source).get_normal_vec();
        let mut specular_coefficient = 0.0;

        let mut diffusion_coefficient = 0.0;

        // ambient is irrespective of direct exposure to light
        let ambient_coefficient = collision_material.get_ambient_coefficient();

        let base_color = collision_material.get_color_at_pos(light_particle_pos);

        let remaining_scene_objects: Vec<&Box<dyn SceneObject>> = scene_objects
            .iter()
            .filter(|scene_object| {
                scene_object.get_position() != nearest_scene_object.get_position()
            })
            .map(|x| *x)
            .collect();

        if self
            .is_direct_exposed_to_light_source(light_particle_pos, remaining_scene_objects.clone())
        {
            // Specular coefficient calculation
            specular_coefficient = f64::powi(
                half_vector.dot_product(surface_normal),
                collision_material.get_specular_adder(),
            );

            // Diffusion coefficient calculation
            diffusion_coefficient = collision_material.get_diffusion_coefficient()
                * surface_normal.dot_product(direction_toward_light_source.get_normal_vec());
        }

        // Color without reflection
        let texture_color = base_color
            .mul(ambient_coefficient)
            .combine_color(self.light_source.color.mul(specular_coefficient))
            .combine_color(base_color.mul(diffusion_coefficient));

        let mut composed_color = texture_color;

        if depth > 0 {
            let reflected_ray = Ray::get_reflected_ray(&ray, surface_normal, light_particle_pos);
            let reflected_ray_color =
                self.get_color_of_ray(reflected_ray, &remaining_scene_objects, depth - 1);
            let reflection_coefficient = collision_material.get_reflection_coefficient();

            composed_color = reflected_ray_color
                .mul(reflection_coefficient)
                .combine_color(composed_color.mul(1.0 - reflection_coefficient));
        }

        return composed_color;
    }
}
