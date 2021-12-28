mod parser;
mod spatial;

pub use crate::parser::point_cloud;
pub use crate::spatial::plane;

pub fn test_both() -> u8{
  point_cloud::get_point() + plane::get_num()
}