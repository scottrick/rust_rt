use std::{fs::File, io::BufWriter, path::Path};

use rust_rt::{Sphere, Vec3};

fn main() {
    println!("Hello, rust_rt!");

    let center_point = Vec3::new(49., 49., 52.);
    let sphere = Sphere::new(center_point, 7.0);

    println!("point: {:?}", sphere.center);
    println!("sphere: {:?}", sphere);

    test_png();
}

fn test_png() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 640;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut buff_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        buff_writer,
        WIDTH.try_into().unwrap(),
        HEIGHT.try_into().unwrap(),
    );

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut pixel_data = [0u8; WIDTH * HEIGHT * 3];

    // populate the pixel data array
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset_start = (x + y * WIDTH) * 3;
            let x_percent = x as f32 / WIDTH as f32;
            let y_percent = y as f32 / HEIGHT as f32;
            pixel_data[offset_start + 0] = (x_percent * (255 as f32)) as u8; //R
            pixel_data[offset_start + 1] = 0; //G
            pixel_data[offset_start + 2] = (y_percent * (255 as f32)) as u8; //B
        }
    }

    writer.write_image_data(&pixel_data).unwrap(); // Save
}
