use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::rc::Rc;

// TODO: rename Hittable

pub enum FaceKind {
  Front,
  Back,
}

pub struct HitRecord {
  pub t: f64,
  pub hit_point: Point3,
  pub normal: Vec3,
  pub face: FaceKind,
  pub material: Rc<dyn Material>,
}

impl HitRecord {
  pub fn new(
    ray: &Ray,
    t: f64,
    hit_point: Point3,
    outward_normal: Vec3,
    material: Rc<dyn Material>,
  ) -> HitRecord {
    let face = match ray.direction.dot(&outward_normal) < 0. {
      true => FaceKind::Front,
      false => FaceKind::Back,
    };
    let normal_multiplier = match face {
      FaceKind::Front => 1.,
      FaceKind::Back => -1.,
    };

    HitRecord {
      t,
      hit_point,
      face,
      normal: normal_multiplier * outward_normal,
      material,
    }
  }
}

// World

pub struct World {
  // pub objects: Vec<&'a dyn Hittable>,
  pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }

  pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit_record: Option<HitRecord> = None;
    let mut closes_so_far = t_max;

    for object in &self.objects {
      match object.as_ref().hit(ray, t_min, closes_so_far) {
        None => (),
        Some(rec) => {
          closes_so_far = rec.t;
          hit_record = Some(rec);
        }
      }
    }

    hit_record
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }
}

// Objects

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let origin_minus_center = ray.origin - self.center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2. * ray.direction.dot(&origin_minus_center);
    let c = origin_minus_center.dot(&origin_minus_center) - self.radius * self.radius;

    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
      return None;
    }
    let mut root = (-b - discriminant.sqrt()) / (2. * a);

    if root < t_min || root > t_max {
      root = (-b + discriminant.sqrt()) / (2. * a);
      if root < t_min || root > t_max {
        return None;
      }
    }

    let t = root;
    let hit_point = ray.at(root);
    let outward_normal = (hit_point - self.center).unit();
    Some(HitRecord::new(
      ray,
      t,
      hit_point,
      outward_normal,
      self.material.clone(),
    ))
  }
}

// Materials

pub struct ScatterRecord {
  pub ray: Ray,
  pub attenuation: Color,
}

pub trait Material {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
  pub albedo: Color,
}

pub struct Metal {
  pub albedo: Color,
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
    let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

    if scatter_direction.near_zero() {
      scatter_direction = hit_record.normal;
    }

    Some(ScatterRecord {
      attenuation: self.albedo,
      ray: Ray {
        origin: hit_record.hit_point,
        direction: scatter_direction,
      },
    })
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
    let scatter_direction =
      ray.direction.unit() - 2. * ray.direction.unit().dot(&hit_record.normal) * hit_record.normal;

    // let unit_ray = ray.direction.unit();
    // let ray_to_normal = hit_record.normal - (-1. * unit_ray);
    // let scatter_direction = (-1. * unit_ray) + (2. * ray_to_normal);

    if scatter_direction.dot(&hit_record.normal) > 0. {
      Some(ScatterRecord {
        attenuation: self.albedo,
        ray: Ray {
          origin: hit_record.hit_point,
          direction: scatter_direction,
        },
      })
    } else {
      None
    }
  }
}
