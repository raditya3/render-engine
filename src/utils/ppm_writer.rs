use std::{fs::File, io::Write};
fn get_pixel_string(pixel: (u8, u8, u8)) -> String {
    return format!("{} {} {}", pixel.0, pixel.1, pixel.2);
}
pub fn write_image(image: Vec<Vec<(u8, u8, u8)>>) {
    let image_height = image.len();
    let image_width = image.get(0).unwrap().len();
    let mut output_string: String = "P3\n".to_owned();
    output_string.push_str(&format!("{} {}\n255\n", image_width, image_height));
    for vec_row in image.iter() {
        for pixel in vec_row {
            let pixel_string = get_pixel_string(*pixel);
            output_string.push_str(&pixel_string);
            output_string.push('\t');
        }
        output_string.push('\n');
    }
    let mut image_file = File::create("output.ppm").expect("Unable to create file");
    image_file
        .write(output_string.as_bytes())
        .expect("Error Writing");

    println!("created file");
}
