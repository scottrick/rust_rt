use std::{fs::File, io::BufWriter, path::Path};

use rust_rt::{Screen, Sphere, Vec3};

fn main() {
    println!("Hello, rust_rt!");

    let center_point = Vec3::new(49., 49., 52.);
    let sphere = Sphere::new(center_point, 7.0);

    println!("point: {:?}", sphere.center);
    println!("sphere: {:?}", sphere);

    test_png();
}

fn test_png() {
    let mut screen = Screen::new(800, 600);

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut buff_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        buff_writer,
        screen.width.try_into().unwrap(),
        screen.height.try_into().unwrap(),
    );

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // populate the pixel data array
    for y in 0..screen.height {
        for x in 0..screen.width {
            let offset_start = (x as usize + y as usize * screen.width as usize) * 3;
            let x_percent = x as f32 / screen.width as f32;
            let y_percent = y as f32 / screen.height as f32;
            screen.pixel_data[offset_start + 0] = (x_percent * (255 as f32)) as u8;
            screen.pixel_data[offset_start + 1] = 0;
            screen.pixel_data[offset_start + 2] = (y_percent * (255 as f32)) as u8;
        }
    }

    writer.write_image_data(&screen.pixel_data).unwrap();
}
