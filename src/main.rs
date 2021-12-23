mod camera;
mod ray;
mod vec3;
mod world;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use camera::Camera;
use ray::Ray;
use vec3::{Color, Point3, Vec3};
use world::{Sphere, World};

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color(0., 0., 0.);
    }
    match world.hit(ray, 0., f64::INFINITY) {
        Some(hit_record) => {
            let reflection_target =
                hit_record.hit_point + hit_record.normal + Vec3::random_in_unit_sphere();

            // let unit_ray = ray.direction.unit();
            // let ray_to_normal = hit_record.normal - (-1. * unit_ray);
            // let reflection_target = hit_record.hit_point + (-1. * unit_ray) + (2. * ray_to_normal);
            return 0.5
                * ray_color(
                    &Ray {
                        origin: hit_record.hit_point,
                        direction: reflection_target - hit_record.hit_point,
                    },
                    world,
                    depth - 1,
                );
        }
        None => (),
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Color(1., 1., 1.) + t * Color(0.5, 0.7, 1.)
}

fn main() {
    let mut rng = SmallRng::from_entropy();
    let aa_uniform = Uniform::from(0.0..1.0);

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

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
                let u = (col as f64 + aa_uniform.sample(&mut rng)) / (image_width - 1) as f64;
                let v = (row as f64 + aa_uniform.sample(&mut rng)) / (image_height - 1) as f64;
                let ray = camera.ray_for(u, v);
                color += ray_color(&ray, &world, max_depth);
            }
            let processed_color = post_process(color, samples_per_pixel);
            println_color(&processed_color);
        }
    }
    eprintln!("\nDone!");
}

fn post_process(color: Color, samples_per_pixel: i32) -> Color {
    // color / samples_per_pixel as f64

    Color(
        (color.0 / samples_per_pixel as f64).sqrt(),
        (color.1 / samples_per_pixel as f64).sqrt(),
        (color.2 / samples_per_pixel as f64).sqrt(),
    )
}

fn println_color(color: &Vec3) {
    let ir = (color.x() * 255.0) as i32;
    let ig = (color.y() * 255.0) as i32;
    let ib = (color.z() * 255.0) as i32;
    println!("{} {} {}", ir, ig, ib);
}
