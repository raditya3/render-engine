use crate::utils::{color::Color, vector::Vector};

pub trait Material {
    fn get_color_at_pos(&self, pos: Vector) -> Color;
    fn get_specular_adder(&self) -> i32;
    fn get_diffusion_coefficient(&self) -> f64;
    fn get_ambient_coefficient(&self) -> f64;
    fn get_reflection_coefficient(&self) -> f64;
}
