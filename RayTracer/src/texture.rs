use crate::interval::Interval;
use crate::vec3::Vec3;
type Color = Vec3;
use crate::rtw::RtwImage;
use crate::perlin::Perlin;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }
    pub fn solid_color(red: f64, green: f64, blue: f64) -> Self {
        let albedo = Color::new(red, green, blue);
        SolidColor::new(albedo)
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.albedo
    }
}


#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn from_texture(scale: f64, even: Arc<dyn Texture + Send + Sync>, odd: Arc<dyn Texture + Send + Sync>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        let even = Arc::new(SolidColor::new(c1));
        let odd = Arc::new(SolidColor::new(c2));

        CheckerTexture::from_texture(scale, even, odd)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = RtwImage::new(filename);
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        if self.image.height <= 0 {
            Color::new(0.0, 1.0, 1.0) // Return solid cyan as a debugging aid
        } else {
            (1.0 / 255.0) * self.image.get_color(u, v)
        }
    }
}

unsafe impl Send for ImageTexture {} 
unsafe impl Sync for ImageTexture {}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
    pub fn with_scale(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

