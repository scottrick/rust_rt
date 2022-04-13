use rust_rt::{Sphere, Vec3};

fn main() {
    println!("Hello, rust_rt!");

    let center_point = Vec3::new(49., 49., 52.);
    let sphere = Sphere::new(center_point, 7.0);

    println!("point: {:?}", sphere.center);
    println!("sphere: {:?}", sphere);
}
