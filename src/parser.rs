/// Parse text files that contain
/// tab delimeted XYZ data
pub mod point_cloud {
    use std::io::{self};
    use crate::primitives::Vec3;

    /// Create a Vec<Vec3> from XYZ data file
    pub fn points_from_file(filename: &str) -> Result<Vec<Vec3>, io::Error> {
        let contents = match std::fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let lines: Vec<&str> = contents.lines().collect();
        let mut output: Vec<Vec3> = Vec::new();

        for line in lines {
            let cols: Vec<&str> = line.split('\t').collect();
            let vals: Vec<f32> = cols
                .iter()
                .map(|x| x.parse().expect("parse error"))
                .collect();
            output.push(Vec3 {
                x: vals[0],
                y: vals[1],
                z: vals[2],
            });
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bad_file() {
        assert!(point_cloud::points_from_file("bad_path.txt").is_err(), "Allowed non-existent file name");
    }
}
