use std::f64::consts::PI;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub direction: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub focal_length: f64,
    pub aspect_ratio: f64,

    horizontal_vector: Vec3,
    vertical_vector: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        direction: Vec3,
        vup: Vec3,
        vfov: f64,
        focal_length: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let mut cam = Camera {
            origin,
            direction,
            vup,
            vfov,
            focal_length,
            aspect_ratio,

            horizontal_vector: Vec3(0., 0., 0.),
            vertical_vector: Vec3(0., 0., 0.),
            lower_left_corner: Vec3(0., 0., 0.),
        };
        cam.update();

        cam
    }

    pub fn ray_for(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner
                + u * self.horizontal_vector
                + v * self.vertical_vector
                - self.origin)
                .unit(),
        }
    }

    pub fn update(&mut self) {
        self.direction = self.direction.unit();

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = viewport_height * self.aspect_ratio;

        let u = self.vup.cross(&self.direction).unit();
        let v = self.direction.cross(&u);

        self.horizontal_vector = viewport_width * u;
        self.vertical_vector = viewport_height * v;
        self.lower_left_corner = self.origin
            - self.horizontal_vector / 2.
            - self.vertical_vector / 2.
            - (self.direction * self.focal_length);
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
