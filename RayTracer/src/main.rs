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
mod constant_medium;

use vec3::Vec3;
type Point3 = Vec3;
type Color = Vec3;
use material::Material;
use std::rc::Rc;
use hit_list::HittableList;
use bvh::BVHNode;
use texture::Texture;
use quad::make_box;
use hit::{RotateY, Translate};
use constant_medium::ConstantMedium;
use rtweekend::random_double;

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
    cam.background = Vec3::new(0.70, 0.80, 1.00);

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
    cam.background = Vec3::new(0.70, 0.80, 1.00);

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
    cam.background = Vec3::new(0.70, 0.80, 1.00);

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
    cam.background = Vec3::new(0.70, 0.80, 1.00);

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
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.render(&world);
}

fn simple_light() {
    let mut world = hit_list::HittableList::new();

    let pertext: Box<dyn Texture> = Box::new(texture::NoiseTexture::with_scale(4.0));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(Rc::new(material::Lambertian::with_texture(pertext.clone())) as Rc<dyn Material>))));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Some(Rc::new(material::Lambertian::with_texture(pertext)) as Rc<dyn Material>))));

    let difflight = Some(Rc::new(material::DiffuseLight::with_color(Vec3::new(4.0, 4.0, 4.0))) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    world.add(Box::new(quad::Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight)));

    let width = 400;
    let height = 400;

    let mut cam = camera::Camera::new(height, width);

    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.render(&world);
}

fn cornell_box() {
    let mut world = hit_list::HittableList::new();

    let red = Some(Rc::new(material::Lambertian::new(Vec3::new(0.65, 0.05, 0.05))) as Rc<dyn Material>);
    let white = Some(Rc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73))) as Rc<dyn Material>);
    let green = Some(Rc::new(material::Lambertian::new(Vec3::new(0.12, 0.45, 0.15))) as Rc<dyn Material>);
    let light = Some(Rc::new(material::DiffuseLight::with_color(Vec3::new(15.0, 15.0, 15.0))) as Rc<dyn Material>);

    world.add(Box::new(quad::Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green)));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red)));
    world.add(Box::new(quad::Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light)));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(quad::Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    // world.addlist(make_box(Point3::new(130.0, 0.0, 65.0), Point3::new(295.0, 165.0, 230.0), white.clone()));
    // world.addlist(make_box(Point3::new(265.0, 0.0, 295.0), Point3::new(430.0, 330.0, 460.0), white));

    let box1 = Box::new(make_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Box::new(make_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);


    let width = 400;
    let height = 400;
    let mut cam = camera::Camera::new(height, width);

    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = hit_list::HittableList::new();

    let red = Some(Rc::new(material::Lambertian::new(Vec3::new(0.65, 0.05, 0.05))) as Rc<dyn Material>);
    let white = Some(Rc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73))) as Rc<dyn Material>);
    let green = Some(Rc::new(material::Lambertian::new(Vec3::new(0.12, 0.45, 0.15))) as Rc<dyn Material>);
    let light = Some(Rc::new(material::DiffuseLight::with_color(Vec3::new(7.0, 7.0, 7.0))) as Rc<dyn Material>);

    world.add(Box::new(quad::Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green)));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red)));
    world.add(Box::new(quad::Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), light)));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(quad::Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Box::new(make_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = Box::new(make_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Box::new(ConstantMedium::new_with_albedo(box1, 0.01, Color::new(0.0, 0.0, 0.0))));
    world.add(Box::new(ConstantMedium::new_with_albedo(box2, 0.01, Color::new(1.0, 1.0, 1.0))));

    let height = 400;
    let width = 400;
    let mut cam = camera::Camera::new(height, width);

    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn final_scene(height: u32, width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = hit_list::HittableList::new();
    let ground = Some(Rc::new(material::Lambertian::new(Vec3::new(0.48, 0.83, 0.53))) as Rc<dyn Material>);

    let boxes_per_side = 20;
    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.addlist(make_box(Point3::new(x0,y0,z0), Point3::new(x1,y1,z1), ground.clone()));
        }
    }

    let mut world = hit_list::HittableList::new();

    world.add(Box::new(bvh::BVHNode::new(&mut boxes1)));

    let light = Some(Rc::new(material::DiffuseLight::with_color(Vec3::new(7.0, 7.0, 7.0))) as Rc<dyn Material>);
    world.add(Box::new(quad::Quad::new(Point3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light)));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Rc::new(material::Lambertian::new(Vec3::new(0.7, 0.3, 0.1))) as Rc<dyn Material>;
    world.add(Box::new(sphere::Sphere::new_moving(center1, center2, 50.0, Some(sphere_material))));

    world.add(Box::new(sphere::Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Some(Rc::new(material::Dielectric::new(1.5)) as Rc<dyn Material>))));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Some(Rc::new(material::Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)) as Rc<dyn Material>))));

    let boundary = Box::new(sphere::Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Some(Rc::new(material::Dielectric::new(1.5)) as Rc<dyn Material>)));
    world.add(boundary.clone());
    world.add(Box::new(ConstantMedium::new_with_albedo(boundary.clone(), 0.2, Color::new(0.2, 0.4, 0.9))));
    let boundary = Box::new(sphere::Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Some(Rc::new(material::Dielectric::new(1.5)) as Rc<dyn Material>)));
    world.add(Box::new(ConstantMedium::new_with_albedo(boundary.clone(), 0.0001, Color::new(1.0, 1.0, 1.0))));

    let emat = Some(Rc::new(material::Lambertian::with_texture(Box::new(texture::ImageTexture::new("earthmap.jpg")))) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat)));

    let pertext = Some(Rc::new(material::Lambertian::with_texture(Box::new(texture::NoiseTexture::with_scale(0.2)))) as Rc<dyn Material>);
    world.add(Box::new(sphere::Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, pertext)));

    let mut boxes2 = hit_list::HittableList::new();
    let white = Some(Rc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73))) as Rc<dyn Material>);
    let ns = 1000;

    for _ in 0..ns {
        boxes2.add(Box::new(sphere::Sphere::new(Vec3::random(0.0, 165.0), 10.0, white.clone())));
    }

    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(bvh::BVHNode::new(&mut boxes2)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = camera::Camera::new(height, width);

    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 0 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 800, 10000, 40),
        _ => final_scene(400, 400, 250, 4),
    }
}
