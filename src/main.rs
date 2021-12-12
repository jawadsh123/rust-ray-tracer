mod ray;
mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let origin_minus_center = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2. * ray.direction.dot(&origin_minus_center);
    let c = origin_minus_center.dot(&origin_minus_center) - radius * radius;

    let discriminant = b * b - 4. * a * c;
    if discriminant > 0. {
        (-b - discriminant.sqrt()) / 2. * a
    } else {
        -1.
    }
}

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3(0., 0., -1.);
    let sphere_radius = 0.5;
    let t = hit_sphere(&sphere_center, sphere_radius, ray);
    if t > 0. {
        let hit_point = ray.at(t);
        let normal = (hit_point - sphere_center).unit();
        return 0.5 * Color(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
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
            let color = ray_color(&ray);
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
