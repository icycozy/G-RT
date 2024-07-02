mod color;
mod vec3;
mod ray;

use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;

use vec3::Vec3;
type Point3 = Vec3;
use ray::Ray;

const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    let path = "output/test.jpg";
    let width = 800;
    let height = 800;
    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let aspect_ratio = 16.0 / 9.0;

    // Calculate the image height, and ensure that it's at least 1.
    let height = (width as f64 / aspect_ratio) as i32;
    let height = if height < 1 { 1 } else { height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u.clone() / (width as f64);
    let pixel_delta_v = viewport_v.clone() / (height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center.clone()
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

    // Render

    println!("P3");
    println!("{} {}", width, height);
    println!("255");


    // 以下是write color和process bar的示例代码
    let mut pixel_color = [255u8; 3];
    for i in 0..height {
        for j in 0..width {
            let pixel_center = pixel00_loc.clone() + (i as f64 * pixel_delta_u.clone()) + (j as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(camera_center.clone(), ray_direction);

            pixel_color = Ray::ray_color(&r);
            write_color(pixel_color, &mut img, i as usize, j as usize);
            bar.inc(1);
        }
    }
    bar.finish();

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
