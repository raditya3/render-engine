use camera::Camera;
use engine::Engine;
use light_source::LightSource;
use material::solid_material;
use shapes::scene_object::SceneObject;
use shapes::sphere::Sphere;
use utils::color::Color;
use utils::ppm_writer;
use utils::vector::Vector;

mod camera;
mod engine;
mod light_source;
mod material;
mod shapes;
mod utils;

fn main() {
    let light_source = LightSource {
        color: Color {
            r: 255,
            g: 255,
            b: 255,
        },
        position: Vector::new(-10.0, 10.0, -10.0),
    };
    let solid_m1 = solid_material::SolidMaterial {
        color: Color::new(255, 0, 0),
        // Ambient color when no light is present
        ambient: 0.5,
        diffusion: 0.6,
        // Value of reflection should be between 0 & 1
        reflection: 0.75,
        // Higher value produce smaller & sharper specular reflection, lower produce more diffused and larger reflection
        specular: 40,
    };
    let solid_m2 = solid_material::SolidMaterial {
        color: Color::new(0, 255, 0),
        ambient: 0.3,
        diffusion: 1.0,
        reflection: 0.0,
        specular: 200,
    };
    let sphere_1 = Box::new(Sphere {
        center: Vector::new(-1.0, -3.0, 15.0),
        radius: 6.0,
        material: Box::new(solid_m1),
    });

    let sphere_2 = Box::new(Sphere {
        center: Vector::new(-1.0, 3.0, 6.0),
        radius: 2.0,
        material: Box::new(solid_m2),
    });

    let camera = Camera::new(1000, 1000, Vector::new(0.0, 0.0, 0.0), 0.5);
    let scene_objects: Vec<Box<dyn SceneObject>> = vec![sphere_1, sphere_2];
    println!("Generating image");
    let render_engine = Engine {
        camera,
        light_source,
        scene_objects,
    };
    let image = render_engine.generate_image();

    // Generates output.ppm file
    ppm_writer::write_image(image);
}
