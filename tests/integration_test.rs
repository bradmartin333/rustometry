#[test]
fn point_cloud_length() {
    let points = rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt");
    assert_eq!(10000, points.unwrap().len());
}

#[test]
fn point_cloud_angle() {
    let points = rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt");
    let plane = rustometry::theta::plane_from_points(points.unwrap());
    let angle = plane.get_theta_degrees();
    assert_eq!(
        rustometry::primitives::Vec2 {
            x: -37.692_65,
            y: 16.778_221
        }
        .round(),
        angle.round()
    );
}
