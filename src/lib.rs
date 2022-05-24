#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

#[derive(Debug)]
pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub pixel_data: Vec<u8>,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixel_data: vec![0u8; width as usize * height as usize * 3],
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub look_dir: Vec3,
    pub up_dir: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, look_dir: Vec3, up_dir: Vec3) -> Self {
        Self {
            pos,
            look_dir,
            up_dir,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
