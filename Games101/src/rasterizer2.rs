use std::collections::HashMap;

use crate::triangle::Triangle;
use nalgebra::{Matrix4, Vector3, Vector4};

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,

    frame_sample: Vec<Vector3<f64>>,
    depth_sample: Vec<f64>,

    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    const SAMPLE_COUNT: usize = 4;

    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r.frame_sample
            .resize((w * h) as usize * Self::SAMPLE_COUNT, Vector3::zeros());
        r.depth_sample
            .resize((w * h) as usize * Self::SAMPLE_COUNT, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_sample.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(
        &mut self,
        pos_buffer: PosBufId,
        ind_buffer: IndBufId,
        col_buffer: ColBufId,
        _typ: Primitive,
    ) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v = vec![
                mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                mvp * to_vec4(buf[i[1]], Some(1.0)),
                mvp * to_vec4(buf[i[2]], Some(1.0)),
            ];

            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
            // self.FXAA(&t);
        }
    }

    // MSAA
    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        let mut tri = [Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z),
                       Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z),
                       Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z)];
        let xmin: f64 = f64::min(t.v[0].x, f64::min(t.v[1].x, t.v[2].x));
        let xmax: f64 = f64::max(t.v[0].x, f64::max(t.v[1].x, t.v[2].x));
        let ymin: f64 = f64::min(t.v[0].y, f64::min(t.v[1].y, t.v[2].y));
        let ymax: f64 = f64::max(t.v[0].y, f64::max(t.v[1].y, t.v[2].y));
        let z: f64 = (t.v[0].z + t.v[1].z + t.v[2].z) / 3.0;
        for x in (xmin as i64)..(xmax as i64) {
            for y in (ymin as i64)..(ymax as i64) {
                let ind = self.get_index(x as usize, y as usize);
                let mut color = Vector3::zeros();
                for sample_x in 0..2 {
                    for sample_y in 0..2 {
                        let pos_x = x as f64 + sample_x as f64 * 0.5;
                        let pos_y = y as f64 + sample_y as f64 * 0.5;
                        let pixel_index = ind * Self::SAMPLE_COUNT + sample_x + sample_y * 2;
                        if inside_triangle(pos_x, pos_y, &tri) {
                            self.frame_sample[pixel_index] = t.get_color();
                        }
                        color += self.frame_sample[pixel_index];
                    }
                }
                color /= Self::SAMPLE_COUNT as f64;
                if color != Vector3::zeros() && z < self.depth_buf[ind] {
                    self.depth_buf[ind] = z;
                    Self::set_pixel(self, &Vector3::new(x as f64, y as f64, 0.0), &color);
                }
            }
        }
    }

    // Assuming luminance can be calculated as a simple average of the RGB components
    fn luminance(color: &Vector3<f64>) -> f64 {
        (color.x + color.y + color.z) / 3.0
    }

    // FXAA implementation for rasterize_triangle
    pub fn FXAA(&mut self, t: &Triangle) {
        // Step 1: Rasterize the triangle normally (filling and edge detection can be done here)
        // This is a placeholder for the existing triangle rasterization logic
        self.basic_rasterize_triangle(t);
        let z = (t.v[0].z + t.v[1].z + t.v[2].z) / 3.0;
        // Step 2: Apply FXAA
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let index = self.get_index(x, y);
                let color = self.frame_sample[index];
                let lum = Self::luminance(&color);

                // Sample neighboring pixels' luminance
                let lum_left = if x > 0 {
                    Self::luminance(&self.frame_sample[self.get_index(x - 1, y)])
                } else {
                    lum
                };
                let lum_right = if x < self.width as usize - 1 {
                    Self::luminance(&self.frame_sample[self.get_index(x + 1, y)])
                } else {
                    lum
                };
                let lum_up = if y > 0 {
                    Self::luminance(&self.frame_sample[self.get_index(x, y - 1)])
                } else {
                    lum
                };
                let lum_down = if y < self.height as usize - 1 {
                    Self::luminance(&self.frame_sample[self.get_index(x, y + 1)])
                } else {
                    lum
                };

                // Calculate gradient
                let gradient = ((lum_left - lum_right).abs() + (lum_up - lum_down).abs()) / 2.0;

                // Determine blend weight based on gradient
                let blend_weight = self.calculate_blend_weight(gradient);

                // Blend colors based on weight
                let color = self.blend_color(x, y, blend_weight);

                // Apply blended color
                Self::set_pixel(self, &Vector3::new(x as f64, y as f64, 0.0), &color);
            }
        }
    }

    // Placeholder for blend weight calculation based on gradient
    fn calculate_blend_weight(&self, gradient: f64) -> f64 {
        // Simplified example: inversely proportional to gradient
        1.0 - gradient.clamp(0.0, 1.0)
    }

    fn blend_color(&self, x: usize, y: usize, weight: f64) -> Vector3<f64> {
        // Simplified example: blend with neighboring pixels
        let mut sum = Vector3::zeros();
        let mut count = 0.0;
        for i in -1..=1 {
            for j in -1..=1 {
                let nx = (x as isize + i).clamp(0, self.width as isize - 1) as usize;
                let ny = (y as isize + j).clamp(0, self.height as isize - 1) as usize;
                let ind = self.get_index(nx, ny);
                sum += self.frame_sample[ind];
                count += 1.0;
            }
        }
        sum * (weight / count)
    }

    fn basic_rasterize_triangle(&mut self, t: &Triangle) {
        let mut tri = [Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z),
                       Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z),
                       Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z)];
        let xmin: f64 = f64::min(t.v[0].x, f64::min(t.v[1].x, t.v[2].x));
        let xmax: f64 = f64::max(t.v[0].x, f64::max(t.v[1].x, t.v[2].x));
        let ymin: f64 = f64::min(t.v[0].y, f64::min(t.v[1].y, t.v[2].y));
        let ymax: f64 = f64::max(t.v[0].y, f64::max(t.v[1].y, t.v[2].y));
        let z: f64 = (t.v[0].z + t.v[1].z + t.v[2].z) / 3.0;
        for x in (xmin as i64)..(xmax as i64) {
            for y in (ymin as i64)..(ymax as i64) {
                let ind = self.get_index(x as usize, y as usize);
                if inside_triangle(x as f64, y as f64, &tri) && z < self.depth_buf[ind] {
                    self.depth_buf[ind] = z;
                    self.frame_sample[ind] = t.get_color();
                }
            }
        }
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    let c1 = (x - v[0].x) * (v[1].y - v[0].y) - (y - v[0].y) * (v[1].x - v[0].x);
    let c2 = (x - v[1].x) * (v[2].y - v[1].y) - (y - v[1].y) * (v[2].x - v[1].x);
    let c3 = (x - v[2].x) * (v[0].y - v[2].y) - (y - v[2].y) * (v[0].x - v[2].x);
    (c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0) || (c1 <= 0.0 && c2 <= 0.0 && c3 <= 0.0)
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
            - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
            - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
            - v[1].x * v[0].y);
    (c1, c2, c3)
}
