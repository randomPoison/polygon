extern crate polygon_math;

use polygon_math::*;

macro_rules! assert_almost_eq {
    ($left: expr, $right: expr) => {
        {
            let difference = $left - $right;
            if !difference.is_zero() {
                panic!("assertion failed: `(left == right)`\n  left; `{:?}`,\n right: `{:?}`,\n  diff: `{:?}`", $left, $right, difference);
            }
        }
    };
}

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

#[test]
fn look_rotation() {
    let result = Orientation::look_rotation(Vector3::FORWARD, Vector3::UP);
    assert_almost_eq!(result.forward(), Vector3::FORWARD);
    assert_almost_eq!(result.up(), Vector3::UP);

    let target_look = Vector3::new(1.0, 0.0, 1.0);
    let result = Orientation::look_rotation(target_look, Vector3::UP);
    assert_almost_eq!(result.forward(), target_look.normalized());
    assert_almost_eq!(result.up(), Vector3::UP);
}
