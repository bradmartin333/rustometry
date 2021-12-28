use rustometry;

#[test]
fn point_cloud_length() {
    let points = rustometry::point_cloud::points_from_file(r"./tests/files/test_point_cloud.txt");
    assert_eq!(10000, points.len());
}
