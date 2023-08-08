use crate::utils::{ray::Ray, vector::Vector};

pub struct Camera {
    pub screen_width: u32,
    pub screen_height: u32,
    pub position: Vector,
    x_step: f64,
    y_step: f64,
    zoom: f64,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, position: Vector, zoom: f64) -> Camera {
        let x0 = -1.0;
        let x1 = 1.0;
        let x_step = (x1 - x0) / ((screen_width - 1) as f64);

        let y0 = -1.0;
        let y1 = 1.0;
        let y_step = (y1 - y0) / ((screen_height - 1) as f64);
        return Camera {
            screen_width,
            screen_height,
            position,
            x_step,
            y_step,
            zoom,
        };
    }
    pub fn get_ray_from_image_point(&self, x: u32, y: u32) -> Ray {
        let x_coord: f64 = -1.0 + (x as f64) * self.x_step;
        let y_coord: f64 = -1.0 + (y as f64) * self.y_step;

        return Ray::new(
            self.position,
            Vector::new(x_coord, y_coord, self.position.z + self.zoom),
        );
    }
}
