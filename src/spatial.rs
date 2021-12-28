pub mod theta {
    use crate::matrices::*;
    use crate::primitives::*;

    pub fn theta_from_points(points: Vec<Vec3>) -> Plane {
        let n = points.len() as f32;
        if n < 3.0 {
            panic!("Less than three points, cannot generate plane");
        }

        let mut sum = Vec3::zero();
        for p in &points {
            sum = &sum + p;
        }
        let centroid = &sum * &Vec3::new(1.0 / n);

        let mut covar = Covariance::default();
        for p in points.iter() {
            let r = p - &centroid;
            covar.add(r);
        }
        covar.average(n);

        Plane {
            centroid,
            normal: covar.weighted_dir().normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn not_enough_points() {
        theta::theta_from_points(vec![crate::primitives::Vec3::zero(); 2]);
    }

    #[test]
    fn zero_plane() {
        assert_eq!(
            crate::primitives::Plane::default().centroid,
            theta::theta_from_points(vec![crate::primitives::Vec3::zero(); 3]).centroid
        );
    }
}
