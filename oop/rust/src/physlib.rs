use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{Add,AddAssign,Sub,SubAssign,Neg,Mul,Div};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_zero() -> Self {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn sqr_magnitude(&self) -> f64 {
        return f64::powi(self.x, 2) + f64::powi(self.y,2) + f64::powi(self.z,2)
    }

    pub fn magnitude(&self) -> f64 {
        return self.sqr_magnitude().sqrt()
    }

    pub fn unit(&self) -> Self {
        let m = self.magnitude();
        return Self { x: self.x / m, y: self.y / m, z: self.z / m }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Self::Output{x: self.x + rhs.x, 
                            y: self.y + rhs.y,
                            z: self.z + rhs.z}
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Self::Output{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return self + -rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.add_assign(-rhs);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        return Self::Output{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return rhs.mul(self);
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        return Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Angles3d {
    theta: f64,
    phi: f64
}

impl Angles3d {
    pub fn new_zero() -> Self {
        return Self { theta: 0.0, phi: 0.0 }
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        let theta_rad = self.theta * PI / 180.0;
        let phi_rad = self.phi * PI / 180.0;
        let x = f64::cos(theta_rad) * f64::sin(phi_rad);
        let y = f64::sin(theta_rad) * f64::sin(phi_rad);
        let z = f64::cos(phi_rad);
        return Vec3 { x, y, z }
    }
}