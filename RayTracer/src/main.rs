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

    let ground_material = Some(Rc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5))) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9 * rtweekend::random_double(0.0, 1.0), 0.2, b as f64 + 0.9 * rtweekend::random_double(0.0, 1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let sphere_material = Some(Rc::new(material::Lambertian::new(albedo)) as Rc<dyn Material>);
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rtweekend::random_double(0.0, 0.5);
                    let sphere_material = Some(Rc::new(material::Metal::new(albedo, fuzz)) as Rc<dyn Material>);
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Some(Rc::new(material::Dielectric::new(1.5)) as Rc<dyn Material>);
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Some(Rc::new(material::Dielectric::new(1.5)) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Some(Rc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1))) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Some(Rc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // Camera
    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat = Point3::new(0.0,0.0,0.0);
    cam.vup = Vec3::new(0.0,1.0,0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}
