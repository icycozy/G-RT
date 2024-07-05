use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::with_values(0.0, 0.999);
    let rbyte = (intensity.clamp(pixel_color.x()) * 255.0) as u8;
    let gbyte = (intensity.clamp(pixel_color.y()) * 255.0) as u8;
    let bbyte = (intensity.clamp(pixel_color.z()) * 255.0) as u8;
    
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([rbyte, gbyte, bbyte]);
}
