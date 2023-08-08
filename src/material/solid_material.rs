use crate::utils::color::Color;
use crate::Vector;

use super::material::Material;

#[derive(Copy, Clone)]
pub struct SolidMaterial {
    pub color: Color,
    pub ambient: f64,
    pub reflection: f64,
    pub specular: i32,
    pub diffusion: f64,
}

impl Material for SolidMaterial {
    fn get_color_at_pos(&self, _pos: Vector) -> Color {
        return self.color.mul(self.ambient);
    }
    fn get_specular_adder(&self) -> i32 {
        return self.specular;
    }

    fn get_diffusion_coefficient(&self) -> f64 {
        return self.diffusion;
    }

    fn get_ambient_coefficient(&self) -> f64 {
        return self.ambient;
    }
    fn get_reflection_coefficient(&self) -> f64 {
        return self.reflection;
    }
}
