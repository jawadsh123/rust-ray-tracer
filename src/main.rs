mod ray;
mod vec3;
mod world;

use ray::Ray;
use vec3::{Color, Point3, Vec3};
use world::{Sphere, World};

fn ray_color(ray: &Ray, world: &World) -> Color {
    match world.hit(ray, 0., f32::INFINITY) {
        Some(hit_record) => {
            return 0.5
                * Color(
                    hit_record.normal.x() + 1.,
                    hit_record.normal.y() + 1.,
                    hit_record.normal.z() + 1.,
                );
        }
        None => (),
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Color(1., 1., 1.) + t * Color(0.5, 0.7, 1.)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // world
    let mut world = World { objects: vec![] };
    world.add(Box::new(Sphere {
        center: Point3(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3(0., -100.5, -1.),
        radius: 100.,
    }));

    // camera
    let viewport_height = 2.;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.;

    let origin = Point3(0., 0., 0.);
    let horizontal_vector = Vec3(viewport_width, 0., 0.);
    let vertical_vector = Vec3(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal_vector / 2. - vertical_vector / 2. - Vec3(0., 0., focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for row in (0..image_height).rev() {
        eprint!("\r Scanlines remaining: {}   ", row);
        for col in 0..image_width {
            let u = col as f32 / (image_width - 1) as f32;
            let v = row as f32 / (image_height - 1) as f32;
            let ray = Ray {
                origin,
                direction: (lower_left_corner + u * horizontal_vector + v * vertical_vector
                    - origin)
                    .unit(),
            };
            let color = ray_color(&ray, &world);
            println_color(&color);
        }
    }
    eprintln!("\nDone!");
}

fn println_color(color: &Vec3) {
    let ir = (color.x() * 255.0) as i32;
    let ig = (color.y() * 255.0) as i32;
    let ib = (color.z() * 255.0) as i32;
    println!("{} {} {}", ir, ig, ib);
}
