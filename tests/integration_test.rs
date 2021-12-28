use rustometry;

#[test]
fn point_cloud_length() {
    let points = rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt");
    assert_eq!(10000, points.len());
}

#[test]
fn point_cloud_angle() {
    let points = rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt");
    let plane = rustometry::theta::theta_from_points(points);
    let angle = plane.get_theta_degrees();
    assert_eq!(
        rustometry::primitives::Vec2 {
            x: -37.692649535954,
            y: 16.7782215821513
        }
        .round(),
        angle.round()
    );
}
