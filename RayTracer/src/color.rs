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
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::with_values(0.0, 0.999);
    let rbyte = (intensity.clamp(r) * 255.0) as u8;
    let gbyte = (intensity.clamp(g) * 255.0) as u8;
    let bbyte = (intensity.clamp(b) * 255.0) as u8;
    
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([rbyte, gbyte, bbyte]);
}
