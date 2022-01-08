use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub horizontal_vector: Vec3,
    pub vertical_vector: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;

        let viewport_height = 2.;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.;

        let origin = Point3(0., 0., 0.);
        let horizontal_vector = Vec3(viewport_width, 0., 0.);
        let vertical_vector = Vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal_vector / 2. - vertical_vector / 2. - Vec3(0., 0., focal_length);

        Camera {
            origin,
            horizontal_vector,
            vertical_vector,
            lower_left_corner,
        }
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
}
