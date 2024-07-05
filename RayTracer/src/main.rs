mod color;
mod vec3;
mod ray;
mod hit;
mod sphere;
mod hit_list;
mod rtweekend;
mod interval;
mod camera;

use vec3::Vec3;
type Point3 = Vec3;

fn main() {
    let width = 800;
    let height = 800;

    // World

    let mut world = hit_list::HittableList::new();

    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = camera::Camera::new(height, width);
    cam.render(&world);
}
