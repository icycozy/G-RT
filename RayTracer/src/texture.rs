use crate::interval::Interval;
use crate::vec3::Vec3;
type Color = Vec3;
use crate::rtw::RtwImage;

pub trait TextureClone {
    fn clone_box(&self) -> Box<dyn Texture>;
}
pub trait Texture: TextureClone {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_box()
    }
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

impl TextureClone for SolidColor {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_texture(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        let even = Box::new(SolidColor::new(c1));
        let odd = Box::new(SolidColor::new(c2));

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

impl TextureClone for CheckerTexture {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
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

    fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        if self.image.height() <= 0 {
            Color::new(0.0, 1.0, 1.0) // Return solid cyan as a debugging aid
        } else {
            let u_clamped = Interval::with_values(0.0, 1.0).clamp(u);
            let v_clamped = Interval::with_values(0.0, 1.0).clamp(v); // Flip V to image coordinates

            let i = (u_clamped * self.image.width() as f64) as usize;
            let j = (v_clamped * self.image.height() as f64) as usize;
            let pixel = self.image.pixel_data(i as i32, j as i32);

            let color_scale = 1.0 / 255.0;
            Color::new(
                color_scale * unsafe { *pixel.offset(0) as f64 },
                color_scale * unsafe { *pixel.offset(1) as f64 },
                color_scale * unsafe { *pixel.offset(2) as f64 },
            )
        }
    }
}

impl TextureClone for ImageTexture {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}
