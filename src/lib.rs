mod parser;
mod spatial;

pub use crate::parser::point_cloud;
pub use crate::spatial::plane;

pub mod primitives {
  #[derive(Debug)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Debug)]
    pub struct Plane {
        pub centroid: Vec3,
        pub normal: Vec3,
    }
}
