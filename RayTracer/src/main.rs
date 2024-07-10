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
mod aabb;
mod bvh;
mod texture;
mod rtw;
mod perlin;
mod quad;

use vec3::Vec3;
type Point3 = Vec3;
type Color = Vec3;
use material::Material;
use std::rc::Rc;
use hit_list::HittableList;
use bvh::BVHNode;
use texture::Texture;

fn bouncing_spheres() {
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
                    let center2 = center + Vec3::new(0.0, rtweekend::random_double(0.0, 0.5), 0.0);
                    world.add(Box::new(sphere::Sphere::new_moving(center, center2, 0.2, sphere_material)));
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


    let world = HittableList::hittable_list(Box::new(BVHNode::new(&mut world)));

    let width = 400;
    let height = 400;
    // Camera
    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat = Point3::new(0.0,0.0,0.0);
    cam.vup = Vec3::new(0.0,1.0,0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = hit_list::HittableList::new();

    let checker:Box<dyn Texture> = Box::new(texture::CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, Some(Rc::new(material::Lambertian::with_texture(checker.clone())) as Rc<dyn Material>))));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, Some(Rc::new(material::Lambertian::with_texture(checker)) as Rc<dyn Material>))));

    let width = 400;
    let height = 400;

    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn earth() {
    let earth_texture = Box::new(texture::ImageTexture::new("earthmap.jpg"));
    let earth_surface:Option<Rc<dyn Material>> = Some(Rc::new(material::Lambertian::with_texture(earth_texture)));
    let globe = Box::new(sphere::Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let width = 400;
    let height = 400;

    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.render(&HittableList::hittable_list(globe));
}

fn perlin_spheres() {
    let mut world = hit_list::HittableList::new();

    let pertext: Box<dyn Texture> = Box::new(texture::NoiseTexture::with_scale(4.0));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(Rc::new(material::Lambertian::with_texture(pertext.clone())) as Rc<dyn Material>))));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Some(Rc::new(material::Lambertian::with_texture(pertext)) as Rc<dyn Material>))));

    let width = 400;
    let height = 400;

    let mut cam = camera::Camera::new(height, width);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world = hit_list::HittableList::new();

    // Materials
    let left_red = Some(Rc::new(material::Lambertian::new(Vec3::new(1.0, 0.2, 0.2))) as Rc<dyn Material>);
    let back_green = Some(Rc::new(material::Lambertian::new(Vec3::new(0.2, 1.0, 0.2))) as Rc<dyn Material>);
    let right_blue = Some(Rc::new(material::Lambertian::new(Vec3::new(0.2, 0.2, 1.0))) as Rc<dyn Material>);
    let upper_orange = Some(Rc::new(material::Lambertian::new(Vec3::new(1.0, 0.5, 0.0))) as Rc<dyn Material>);
    let lower_teal = Some(Rc::new(material::Lambertian::new(Vec3::new(0.2, 0.8, 0.8))) as Rc<dyn Material>);

    // Quads
    world.add(Box::new(quad::Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Box::new(quad::Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Box::new(quad::Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Box::new(quad::Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Box::new(quad::Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal)));

    let width = 400;
    let height = 400;

    let mut cam = camera::Camera::new(height, width);

    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 5 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        _ => (),
    }
}
