mod parser;
mod spatial;

pub use crate::parser::point_cloud;
pub use crate::spatial::theta;

pub mod primitives {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(PartialEq, Debug)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }

    impl Vec2 {
        pub fn round(&self) -> Vec2 {
            Vec2 {
                x: ((self.x * 1e3).floor()) / 1e3,
                y: ((self.y * 1e3).floor()) / 1e3,
            }
        }
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vec3 {
        pub fn new(value: f32) -> Vec3 {
            Vec3 {
                x: value,
                y: value,
                z: value,
            }
        }

        pub fn zero() -> Vec3 {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        fn one() -> Vec3 {
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
        }

        fn x_dir() -> Vec3 {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        }

        fn y_dir() -> Vec3 {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        }

        pub fn dot(&self, axis: &Vec3) -> f32 {
            self.x * axis.x + self.y * axis.y + self.z * axis.z
        }

        fn dot_one(&self) -> f32 {
            self.dot(&Vec3::one())
        }

        pub fn normalize(&self) -> Vec3 {
            let len = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
            self / &Vec3::new(len)
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            };
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            };
        }
    }

    impl MulAssign for Vec3 {
        fn mul_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            };
        }
    }

    impl DivAssign for Vec3 {
        fn div_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            };
        }
    }

    impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
        type Output = Vec3;

        fn add(self, other: &'b Vec3) -> Vec3 {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
        type Output = Vec3;

        fn sub(self, other: &'b Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
        type Output = Vec3;

        fn mul(self, other: &'b Vec3) -> Vec3 {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }

    impl<'a, 'b> Div<&'b Vec3> for &'a Vec3 {
        type Output = Vec3;

        fn div(self, other: &'b Vec3) -> Vec3 {
            Vec3 {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
    }

    #[derive(Debug)]
    pub struct Plane {
        pub centroid: Vec3,
        pub normal: Vec3,
    }

    impl Default for Plane {
        fn default() -> Self {
            Plane {
                centroid: Vec3::zero(),
                normal: Vec3::zero(),
            }
        }
    }

    impl Plane {
        fn distance(&self) -> f32 {
            (&self.centroid * &self.normal).dot_one()
        }

        fn project(&self, point: Vec3) -> Vec3 {
            let num = self.normal.dot(&point);
            let vec = &self.normal * &Vec3::new(num + self.distance());
            &point - &vec
        }

        fn get_theta_radians(&self) -> Vec2 {
            let projected_start = self.project(Vec3::zero());
            let x_projected_finish = self.project(Vec3::x_dir());
            let y_projected_finish = self.project(Vec3::y_dir());
            Vec2 {
                x: -1.0
                    * ((x_projected_finish.z - projected_start.z)
                        / (x_projected_finish.x - projected_start.x))
                        .atan(),
                y: -1.0
                    * ((y_projected_finish.z - projected_start.z)
                        / (y_projected_finish.y - projected_start.y))
                        .atan(),
            }
        }

        pub fn get_theta_degrees(&self) -> Vec2 {
            let radians = self.get_theta_radians();
            Vec2 {
                x: radians.x * (180.0 / std::f32::consts::PI),
                y: radians.y * (180.0 / std::f32::consts::PI),
            }
        }
    }
}

pub mod matrices {
    use crate::primitives::Vec3;

    pub struct Covariance {
        pub xx: f32,
        pub xy: f32,
        pub xz: f32,
        pub yy: f32,
        pub yz: f32,
        pub zz: f32,
    }

    impl Default for Covariance {
        fn default() -> Self {
            Covariance {
                xx: 0.0,
                xy: 0.0,
                xz: 0.0,
                yy: 0.0,
                yz: 0.0,
                zz: 0.0,
            }
        }
    }

    impl Covariance {
        pub fn add(&mut self, r: Vec3) {
            *self = Self {
                xx: self.xx + (r.x * r.x),
                xy: self.xy + (r.x * r.y),
                xz: self.xz + (r.x * r.z),
                yy: self.yy + (r.y * r.y),
                yz: self.yz + (r.y * r.z),
                zz: self.zz + (r.z * r.z),
            }
        }

        pub fn average(&mut self, n: f32) {
            *self = Self {
                xx: self.xx / n,
                xy: self.xy / n,
                xz: self.xz / n,
                yy: self.yy / n,
                yz: self.yz / n,
                zz: self.zz / n,
            }
        }

        fn det_x(&self) -> f32 {
            self.yy * self.zz - self.yz * self.yz
        }

        fn det_y(&self) -> f32 {
            self.xx * self.zz - self.xz * self.xz
        }

        fn det_z(&self) -> f32 {
            self.xx * self.yy - self.xy * self.xy
        }

        pub fn weighted_dir(&self) -> Vec3 {
            // exclude symmetries
            let mut out = Vec3::zero();

            {
                let det_x = self.det_x();
                let axis_dir = Vec3 {
                    x: det_x,
                    y: self.xz * self.yz - self.xy * self.zz,
                    z: self.xy * self.yz - self.xz * self.yy,
                };
                let mut weight = det_x * det_x;
                if out.dot(&axis_dir) < 0.0 {
                    weight = -weight;
                }
                out += &axis_dir * &Vec3::new(weight);
            }

            {
                let det_y = self.det_y();
                let axis_dir = Vec3 {
                    x: self.xz * self.yz - self.xy * self.zz,
                    y: det_y,
                    z: self.xy * self.xz - self.yz * self.xx,
                };
                let mut weight = det_y * det_y;
                if out.dot(&axis_dir) < 0.0 {
                    weight = -weight;
                }
                out += &axis_dir * &Vec3::new(weight);
            }

            {
                let det_z = self.det_z();
                let axis_dir = Vec3 {
                    x: self.xy * self.yz - self.xz * self.yy,
                    y: self.xy * self.xz - self.yz * self.xx,
                    z: det_z,
                };
                let mut weight = det_z * det_z;
                if out.dot(&axis_dir) < 0.0 {
                    weight = -weight;
                }
                out += &axis_dir * &Vec3::new(weight);
            }

            out
        }
    }
}
