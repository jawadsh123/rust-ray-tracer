#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);
pub use Vec3 as Point3;
pub use Vec3 as Color;

impl Vec3 {
  pub fn x(&self) -> f32 {
    self.0
  }
  pub fn y(&self) -> f32 {
    self.1
  }
  pub fn z(&self) -> f32 {
    self.2
  }
  pub fn dot(&self, rhs: &Vec3) -> f32 {
    (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
  }
  pub fn cross(&self, rhs: &Vec3) -> f32 {
    (self.1 * rhs.2 - self.2 * rhs.1)
      + (self.2 * rhs.0 - self.0 * rhs.2)
      + (self.0 * rhs.1 - self.1 * rhs.0)
  }
  pub fn unit(&self) -> Vec3 {
    *self / self.length()
  }
  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn length_squared(&self) -> f32 {
    (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
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

impl std::ops::Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f32) -> Vec3 {
    Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
  }
}

impl std::ops::Mul<Vec3> for f32 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
  }
}

impl std::ops::Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f32) -> Vec3 {
    Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
  }
}
