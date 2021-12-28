pub mod point_cloud {
    use crate::primitives::Vec3;

    pub fn points_from_file(filename: &str) -> Vec<Vec3> {
        let contents =
            std::fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines: Vec<&str> = contents.lines().collect();
        let mut output: Vec<Vec3> = Vec::new();

        for line in lines {
            let cols: Vec<&str> = line.split('\t').collect();
            let vals: Vec<f64> = cols
                .iter()
                .map(|x| x.parse().expect("parse error"))
                .collect();
            output.push(Vec3 {
                x: vals[0],
                y: vals[1],
                z: vals[2],
            });
        }

        output
    }
}
