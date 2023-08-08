#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
fn f64_u8_color_clamp(val: f64) -> u8 {
    if val >= 255.0 {
        return 255;
    } else if val <= 0.0 {
        return 0;
    }
    return val as u8;
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        return Color { r, g, b };
    }

    pub fn mul(self, rhs: f64) -> Color {
        return Color {
            b: f64_u8_color_clamp((self.b as f64) * rhs),
            r: f64_u8_color_clamp((self.r as f64) * rhs),
            g: f64_u8_color_clamp((self.g as f64) * rhs),
        };
    }

    pub fn combine_color(&self, other: Color) -> Color {
        let r = f64_u8_color_clamp(self.r as f64 + other.r as f64);
        let g = f64_u8_color_clamp(self.g as f64 + other.g as f64);
        let b = f64_u8_color_clamp(self.b as f64 + other.b as f64);
        return Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        };
    }
}
