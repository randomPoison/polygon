extern crate polygon_math;

use polygon_math::*;

#[test]
fn identity_multiplication() {
    // Test that multiplication against the identity quaternion does yields the correct result.
    let identity = Quaternion::identity();
    assert_eq!(identity * identity, identity);

    let quat = Orientation::axis_angle(Vector3::new(1.0, 0.0, 0.0), PI);
    assert_eq!(identity * quat.0, quat.0);
    assert_eq!(quat.0 * identity, quat.0);
}

#[test]
fn into_matrix4() {
    assert_eq!(Matrix4::identity(), Orientation::new().into());

    assert_eq!(Matrix4::rotation(PI, 0.0, 0.0), Orientation::axis_angle(Vector3::new(1.0, 0.0, 0.0), PI).into());
    assert_eq!(Matrix4::rotation(0.0, PI, 0.0), Orientation::axis_angle(Vector3::new(0.0, 1.0, 0.0), PI).into());
    assert_eq!(Matrix4::rotation(0.0, 0.0, PI), Orientation::axis_angle(Vector3::new(0.0, 0.0, 1.0), PI).into());

    assert_eq!(Matrix4::rotation(PI * 0.5, 0.0, 0.0), Orientation::axis_angle(Vector3::new(1.0, 0.0, 0.0), PI * 0.5).into());
    assert_eq!(Matrix4::rotation(0.0, PI * 0.5, 0.0), Orientation::axis_angle(Vector3::new(0.0, 1.0, 0.0), PI * 0.5).into());
    assert_eq!(Matrix4::rotation(0.0, 0.0, PI * 0.5), Orientation::axis_angle(Vector3::new(0.0, 0.0, 1.0), PI * 0.5).into());

    assert_eq!(Matrix4::rotation(0.5, 0.0, 0.0), Orientation::axis_angle(Vector3::new(1.0, 0.0, 0.0), 0.5).into());
    assert_eq!(Matrix4::rotation(0.0, 0.5, 0.0), Orientation::axis_angle(Vector3::new(0.0, 1.0, 0.0), 0.5).into());
    assert_eq!(Matrix4::rotation(0.0, 0.0, 0.5), Orientation::axis_angle(Vector3::new(0.0, 0.0, 1.0), 0.5).into());
}
