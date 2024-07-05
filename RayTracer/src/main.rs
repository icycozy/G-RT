mod color;
mod vec3;
mod ray;
mod hit;
mod sphere;
mod hit_list;
mod rtweekend;
mod interval;
mod camera;
mod material;

use vec3::Vec3;
type Point3 = Vec3;
use material::Material;
use std::rc::Rc;

fn main() {
    let width = 800;
    let height = 800;

    // World

    let mut world = hit_list::HittableList::new();

    let material_ground = Some(Rc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0))) as Rc<dyn Material>);
    let material_center = Some(Rc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5))) as Rc<dyn Material>);
    let material_left = Some(Rc::new(material::Dielectric::new(1.50)) as Rc<dyn Material>);
    let material_bubble = Some(Rc::new(material::Dielectric::new(1.00 / 1.50)) as Rc<dyn Material>);
    let material_right = Some(Rc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)) as Rc<dyn Material>);

    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0,2.0,1.0);
    cam.lookat = Point3::new(0.0,0.0,-1.0);
    cam.vup = Vec3::new(0.0,1.0,0.0);
    cam.render(&world);
}
