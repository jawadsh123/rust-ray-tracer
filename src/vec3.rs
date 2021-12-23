use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub use Vec3 as Point3;
pub use Vec3 as Color;

impl Vec3 {
  pub fn x(&self) -> f64 {
    self.0
  }
  pub fn y(&self) -> f64 {
    self.1
  }
  pub fn z(&self) -> f64 {
    self.2
  }
  pub fn dot(&self, rhs: &Vec3) -> f64 {
    (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
  }
  pub fn cross(&self, rhs: &Vec3) -> f64 {
    (self.1 * rhs.2 - self.2 * rhs.1)
      + (self.2 * rhs.0 - self.0 * rhs.2)
      + (self.0 * rhs.1 - self.1 * rhs.0)
  }
  pub fn unit(&self) -> Vec3 {
    *self / self.length()
  }
  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }
  pub fn length_squared(&self) -> f64 {
    (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
  }
  pub fn random_in_range(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3(
      rng.gen_range(min..max),
      rng.gen_range(min..max),
      rng.gen_range(min..max),
    )
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    let mut vec = Vec3::random_in_range(-1., 1.);
    while vec.length_squared() >= 1. {
      vec = Vec3::random_in_range(-1., 1.);
    }

    vec
  }
}

impl std::ops::Add for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl std::ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Vec3) {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self.2 += rhs.2;
  }
}

impl std::ops::Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

impl std::ops::Mul for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}

impl std::ops::Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f64) -> Vec3 {
    Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
  }
}

impl std::ops::Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
  }
}

impl std::ops::Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f64) -> Vec3 {
    Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
  }
}
