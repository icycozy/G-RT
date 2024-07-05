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
    pub samples_per_pixel: u32,
    pixel_samples_scale: f64,
    pub max_depth: u32,
    pub vfov: f64, // Vertical view angle (field of view)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3, // Point camera is looking at
    pub vup: Vec3, // Camera-relative "up" direction
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

const AUTHOR: &str = "name";

pub fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

impl Camera {
    pub fn new(image_height: u32, image_width: u32) -> Self {
        Camera {
            image_width,
            image_height,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            pixel_samples_scale: 1.0 / 10.0, 
            max_depth : 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn initialize(&mut self) {
        let center = self.lookfrom.clone();

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(self.w).unit();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width;
        let viewport_v = -1.0 * self.v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.center = center;
        self.pixel00_loc = pixel00_loc;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        let path = "output/defocus_blur.jpg";
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
                    pixel_color = pixel_color + r.ray_color(self.max_depth, &world);
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

        let ray_origin = if (self.defocus_angle <= 0.0) { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double(0.0, 1.0) - 0.5, random_double(0.0, 1.0) - 0.5, 0.0)
    }
}