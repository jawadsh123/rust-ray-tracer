use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// TODO: rename Hittable

pub enum FaceKind {
  Front,
  Back,
}

pub struct HitRecord {
  pub t: f32,
  pub hit_point: Point3,
  pub normal: Vec3,
  pub face: FaceKind,
}

impl HitRecord {
  pub fn new(ray: &Ray, t: f32, hit_point: Point3, outward_normal: Vec3) -> HitRecord {
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

  pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
  pub center: Point3,
  pub radius: f32,
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
    Some(HitRecord::new(ray, t, hit_point, outward_normal))
  }
}
