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
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Materials
    let mat_ground = Rc::new(Lambertian {
        albedo: Color(0.8, 0.8, 0.0),
    });
    let mat_left = Rc::new(Lambertian {
        albedo: Color(0.7, 0.3, 0.2),
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
        material: mat_ground,
    }));
    world.add(Box::new(Sphere {
        center: Point3(0., 0., -1.),
        radius: 0.5,
        material: mat_center,
    }));
    world.add(Box::new(Sphere {
        center: Point3(1., 0., -1.),
        radius: 0.5,
        material: mat_right,
    }));
    world.add(Box::new(Sphere {
        center: Point3(-1., 0., -1.),
        radius: 0.5,
        material: mat_left,
    }));

    // camera
    let camera = Camera::new();

    let mut image_buffer = vec![vec![Color(0., 0., 0.); image_width]; image_height];

    for sample in 0..samples_per_pixel {
        eprint!("\r Samples remaining: {}   ", samples_per_pixel - sample);
        for row in (0..image_height).rev() {
            for col in 0..image_width {
                let mut color = Color(0., 0., 0.);

                let u = (col as f64 + aa_uniform.sample(&mut rng)) / (image_width - 1) as f64;
                let v = (row as f64 + aa_uniform.sample(&mut rng)) / (image_height - 1) as f64;
                let ray = camera.ray_for(u, v);
                color += ray_color(&ray, &world, max_depth);

                if sample == 0 {
                    image_buffer[row][col].0 = (color.x()).sqrt();
                    image_buffer[row][col].1 = (color.y()).sqrt();
                    image_buffer[row][col].2 = (color.z()).sqrt();
                } else {
                    image_buffer[row][col].0 =
                        acc_color_channel(image_buffer[row][col].0, color.x(), sample);
                    image_buffer[row][col].1 =
                        acc_color_channel(image_buffer[row][col].1, color.y(), sample);
                    image_buffer[row][col].2 =
                        acc_color_channel(image_buffer[row][col].2, color.z(), sample);
                }
            }
        }
    }

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    image_buffer.into_iter().for_each(|row| {
        row.into_iter().for_each(|pixel| {
            println!(
                "{} {} {}",
                (pixel.0 * 255.0) as i32,
                (pixel.1 * 255.0) as i32,
                (pixel.2 * 255.0) as i32
            )
        })
    });

    eprintln!("\nDone!");
}

// moving average + gamma correction
fn acc_color_channel(acc: f64, new_val: f64, iteration: i32) -> f64 {
    let acc_component = iteration as f64 * acc * acc;
    let new_average = (new_val + acc_component) / (iteration + 1) as f64;

    new_average.sqrt()
}
