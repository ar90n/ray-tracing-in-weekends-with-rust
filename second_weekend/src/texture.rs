use image::{Rgb, RgbImage};

use super::perlin::Perlin;
use super::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3,
}
impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: RgbImage,
}

impl ImageTexture {
    pub fn new(data: RgbImage) -> Self {
        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let i = (u.max(0.0).min(1.0) * (self.data.width() - 1) as f32).floor() as u32;
        let j = ((1.0 - v).max(0.0).min(1.0) * (self.data.height() - 1) as f32).floor() as u32;
        let Rgb([r, g, b]) = self.data.get_pixel(i, j);
        Vec3::new(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0)
    }
}
