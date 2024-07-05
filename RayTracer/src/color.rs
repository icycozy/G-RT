use image::RgbImage;
use crate::interval::Interval;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
    let r = pixel_color[0] as f64;
    let g = pixel_color[1] as f64;
    let b = pixel_color[2] as f64;

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::with_values(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;
    
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([rbyte, gbyte, bbyte]);
    // Write the translated [0,255] value of each color component.
}
