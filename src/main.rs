mod camera;
mod ray;
mod vec3;
mod world;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::rc::Rc;

use camera::Camera;
use ray::Ray;
use vec3::{Color, Point3, Vec3};
use world::{Dielectric, Lambertian, Metal, Sphere, World};

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color(0., 0., 0.);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        match (*hit_record.material).scatter(ray, &hit_record) {
            Some(scatter_record) => {
                return scatter_record.attenuation
                    * ray_color(&scatter_record.ray, world, depth - 1)
            }
            None => return Color(0., 0., 0.),
        }
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
    let image_width = 480;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Materials
    let mat_ground = Rc::new(Lambertian {
        albedo: Color(0.8, 0.8, 0.0),
    });
    let mat_left = Rc::new(Lambertian {
        albedo: Color(0.7, 0.3, 0.2),
    });
    let mat_back = Rc::new(Lambertian {
        albedo: Color(0.2, 0.3, 0.7),
    });
    let mat_right = Rc::new(Metal {
        albedo: Color(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });
    let mat_center = Rc::new(Dielectric {
        ior: 1.5,
        rng: SmallRng::from_entropy(),
    });

    // world
    let mut world = World { objects: vec![] };
    world.add(Box::new(Sphere {
        center: Point3(0., -100.5, -1.),
        radius: 100.,
        material: mat_ground.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Point3(0., 0., -1.),
        radius: 0.5,
        material: mat_center.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Point3(1., 0., -1.),
        radius: 0.5,
        material: mat_right.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Point3(0., 0., -4.0),
        radius: 0.5,
        material: mat_back.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Point3(-1., 0., -1.),
        radius: 0.5,
        material: mat_left.clone(),
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
