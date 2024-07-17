use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero.
    if r.is_nan() { r = 0.0; }
    if g.is_nan() { g = 0.0; }
    if b.is_nan() { b = 0.0; }

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::with_values(0.0, 0.999);
    let rbyte = (intensity.clamp(r) * 255.0) as u8;
    let gbyte = (intensity.clamp(g) * 255.0) as u8;
    let bbyte = (intensity.clamp(b) * 255.0) as u8;
    
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([rbyte, gbyte, bbyte]);
}
