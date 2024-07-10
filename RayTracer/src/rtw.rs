use std::env;
use std::ffi::CString;
use std::fmt::format;
use std::ptr;

#[repr(C)]

#[derive(Clone)]
pub struct RtwImage {
    bytes_per_pixel: i32,
    fdata: *mut f32,
    bdata: *mut u8,
    image_width: i32,
    image_height: i32,
    bytes_per_scanline: i32,
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        let filename = CString::new(image_filename).expect("Failed to create CString");
        let imagedir = env::var("RtwImageS").ok();

        let mut image = RtwImage {
            bytes_per_pixel: 3,
            fdata: ptr::null_mut(),
            bdata: ptr::null_mut(),
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0,
        };

        // Hunt for the image file in some likely locations.
        if let Some(imagedir) = imagedir {
            let path = format!("{}/{}", imagedir, image_filename);
            if image.load(&path) {
                return image;
            }
        }

        if image.load(image_filename) {
            return image;
        }

        let paths = [
            format!("images/{}", image_filename),
            format!("../images/{}", image_filename),
            format!("../../images/{}", image_filename),
            format!("../../../images/{}", image_filename),
            format!("../../../../images/{}", image_filename),
            format!("../../../../../images/{}", image_filename),
            format!("../../../../../../images/{}", image_filename),
        ];

        // for path in paths.iter() {
        //     if image.load(path) {
        //         println!("Loaded image from {}", path);
        //         return image;
        //     }
        // }

        let path = format!("images/1.jpg");
        if image.load(&path) {
            return image;
        }

        image
    }

    fn load(&mut self, filename: &str) -> bool {
        let c_filename = CString::new(filename).expect("Failed to create CString");
        let mut n = self.bytes_per_pixel;

        unsafe {
            self.fdata = stbi_loadf(c_filename.as_ptr(), &mut self.image_width, &mut self.image_height, &mut n, self.bytes_per_pixel);
        }

        if self.fdata.is_null() {
            return false;
        }

        self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
        self.convert_to_bytes();
        true
    }

    pub fn width(&self) -> i32 {
        if self.fdata.is_null() {
            0
        } else {
            self.image_width
        }
    }

    pub fn height(&self) -> i32 {
        if self.fdata.is_null() {
            0
        } else {
            self.image_height
        }
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> *const u8 {
        let magenta: [u8; 3] = [255, 0, 255];

        if self.bdata.is_null() {
            return magenta.as_ptr();
        }

        let x = self.clamp(x, 0, self.image_width);
        let y = self.clamp(y, 0, self.image_height);

        unsafe {
            self.bdata.add((y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize)
        }
    }

    pub fn clamp(&self, x: i32, low: i32, high: i32) -> i32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }

    fn float_to_byte(&self, value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (256.0 * value) as u8
        }
    }

    fn convert_to_bytes(&mut self) {
        let total_bytes = (self.image_width * self.image_height * self.bytes_per_pixel) as usize;
        self.bdata = vec![0u8; total_bytes].into_boxed_slice().as_mut_ptr();

        unsafe {
            let mut bptr = self.bdata;
            let mut fptr = self.fdata;
            for _ in 0..total_bytes {
                *bptr = self.float_to_byte(*fptr);
                bptr = bptr.add(1);
                fptr = fptr.add(1);
            }
        }
    }
}

extern "C" {
    fn stbi_loadf(filename: *const i8, width: *mut i32, height: *mut i32, channels: *mut i32, desired_channels: i32) -> *mut f32;
}