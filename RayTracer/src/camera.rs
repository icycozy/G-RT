use crate::ray::Ray;
use crate::vec3::Vec3;
type Point3 = Vec3;
use crate::hit_list::HittableList;
use crate::color::write_color;
use indicatif::ProgressBar;
use image::{ImageBuffer, RgbImage};
use std::fs::File;
use crate::rtweekend::random_double;

pub struct Camera {
    image_width: u32,   // Rendered image width in pixel count
    image_height: u32,  // Rendered image height
    center: Point3,     // Camera center
    pixel00_loc: Point3,// Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
}

const AUTHOR: &str = "name";

pub fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

impl Camera {
    pub fn new(image_height: u32, image_width: u32) -> Self {
        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center.clone()
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

        let samples_per_pixel = 100;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64), 
        }
    }

    pub fn render(&self, world: &HittableList) {
        let path = "output/test.jpg";
        let quality = 60;

        let bar: ProgressBar = if is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
    
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + r.ray_color(&world);
                }
                pixel_color = pixel_color * self.pixel_samples_scale;
                
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

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
                          + ((i as f64 + offset.x()) * self.pixel_delta_u)
                          + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double(0.0, 1.0) - 0.5, random_double(0.0, 1.0) - 0.5, 0.0)
    }
}