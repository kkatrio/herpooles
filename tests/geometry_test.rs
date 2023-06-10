use approx::assert_abs_diff_eq;
use herpooles::geometry;

#[test]
fn mul_scalar_vec() {
    let v = geometry::Vector { x: 2.0, y: 3.0 };
    let result = v.clone() * 3.0;
    let expected = geometry::Vector { x: 6.0, y: 9.0 };
    assert_abs_diff_eq!(result.x, expected.x, epsilon = std::f32::EPSILON);
    assert_abs_diff_eq!(result.y, expected.y, epsilon = std::f32::EPSILON);
}

#[test]
fn add_vec_to_point() {
    let v = geometry::Vector { x: 2.0, y: 3.0 };
    let p = geometry::Point { x: 1.0, y: 1.0 };
    let res = p + v;
    let expected = geometry::Point { x: 3.0, y: 4.0 };
    assert_abs_diff_eq!(res.x, expected.x, epsilon = std::f32::EPSILON);
    assert_abs_diff_eq!(res.y, expected.y, epsilon = std::f32::EPSILON);
}

#[test]
fn unit_vec() {
    let v = geometry::Vector { x: 4.0, y: 3.0 };
    let expected = geometry::Vector { x: 0.8, y: 0.6 };
    assert_abs_diff_eq!(v.unit_vec().x, expected.x, epsilon = std::f32::EPSILON);
    assert_abs_diff_eq!(v.unit_vec().y, expected.y, epsilon = std::f32::EPSILON);
}
