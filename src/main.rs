mod camera;
mod ray;
mod vec3;
mod world;

use rand::Rng;

use camera::Camera;
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
    let mut rng = rand::thread_rng();

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 4;

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
    let camera = Camera::new();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for row in (0..image_height).rev() {
        eprint!("\r Scanlines remaining: {}   ", row);
        for col in 0..image_width {
            let mut color = Color(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (col as f32 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f32;
                let v = (row as f32 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f32;
                let ray = camera.ray_for(u, v);
                color += ray_color(&ray, &world);
            }
            let processed_color = post_process(color, samples_per_pixel);
            println_color(&processed_color);
        }
    }
    eprintln!("\nDone!");
}

fn post_process(color: Color, samples_per_pixel: i32) -> Color {
    color / samples_per_pixel as f32
}

fn println_color(color: &Vec3) {
    let ir = (color.x() * 255.0) as i32;
    let ig = (color.y() * 255.0) as i32;
    let ib = (color.z() * 255.0) as i32;
    println!("{} {} {}", ir, ig, ib);
}
