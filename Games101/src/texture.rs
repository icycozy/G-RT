#![allow(warnings)]
use nalgebra::{Vector3};

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

#[derive(Clone)]

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        println!("Loading texture: {}", name);
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        // println!("Color: {:?}", color);
        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        // Ensure u and v are within the [0, 1] range
        u = u.clamp(0.0, 1.0);
        v = v.clamp(0.0, 1.0);

        // Calculate the surrounding pixel indices
        let u_img = u * (self.width as f64 - 1.0);
        let v_img = (1.0 - v) * (self.height as f64 - 1.0);
        let x = u_img.floor() as i32;
        let y = v_img.floor() as i32;
        let x1 = (u_img.ceil() as i32).min(self.width as i32 - 1);
        let y1 = (v_img.ceil() as i32).min(self.height as i32 - 1);

        // Calculate the fractional parts of u and v
        let u_ratio = u_img - x as f64;
        let v_ratio = v_img - y as f64;
        let u_opposite = 1.0 - u_ratio;
        let v_opposite = 1.0 - v_ratio;

        // Retrieve the colors of the four surrounding pixels
        let top_left: &VecN<u8, 3> = self.img_data.at_2d(y, x).unwrap();
        let top_right: &VecN<u8, 3> = self.img_data.at_2d(y, x1).unwrap();
        let bottom_left: &VecN<u8, 3> = self.img_data.at_2d(y1, x).unwrap();
        let bottom_right: &VecN<u8, 3> = self.img_data.at_2d(y1, x1).unwrap();

        // Perform the bilinear interpolation
        let color_top = Vector3::new(
            (top_left[2] as f64 * u_opposite + top_right[2] as f64 * u_ratio),
            (top_left[1] as f64 * u_opposite + top_right[1] as f64 * u_ratio),
            (top_left[0] as f64 * u_opposite + top_right[0] as f64 * u_ratio),
        );
        let color_bottom = Vector3::new(
            (bottom_left[2] as f64 * u_opposite + bottom_right[2] as f64 * u_ratio),
            (bottom_left[1] as f64 * u_opposite + bottom_right[1] as f64 * u_ratio),
            (bottom_left[0] as f64 * u_opposite + bottom_right[0] as f64 * u_ratio),
        );

        // println!("Color: {:?}", color_top);
        Vector3::new(
            (color_top.x * v_opposite + color_bottom.x * v_ratio),
            (color_top.y * v_opposite + color_bottom.y * v_ratio),
            (color_top.z * v_opposite + color_bottom.z * v_ratio),
        )
    }
}