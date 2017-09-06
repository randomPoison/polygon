extern crate polygon_math;

use polygon_math::matrix::Matrix4;

#[test]
fn matrix_equality()
{
    let identity_1 = Matrix4::identity();
    let mut identity_2 = Matrix4::identity();

    assert!(identity_1 == identity_1); // self equality
    assert!(identity_1 == identity_2); // two identity matrices

    identity_2[0][0] = 5.0;
    assert!(identity_1 != identity_2);
}

#[test]
#[should_panic(expected = "Cannot get matrix row 4 in a 4x4 matrix")]
fn matrix_index_bounds() {
    let matrix = Matrix4::identity();
    matrix[4];
}

#[test]
#[should_panic(expected = "Cannot get matrix row 4 in a 4x4 matrix")]
#[allow(unused_mut)]
fn matrix_mut_index_bounds() {
    let mut matrix = Matrix4::identity();
    matrix[4];
}

#[test]
fn matrix_identity() {
    let identity = Matrix4::identity();

    assert!(identity[0][0] == 1.0);
    assert!(identity[1][1] == 1.0);
    assert!(identity[2][2] == 1.0);
    assert!(identity[3][3] == 1.0);
}

#[test]
fn matrix_translation()
{
    let identity = Matrix4::identity();

    let translation_1 = Matrix4::translation(0.0, 0.0, 0.0);
    let translation_2 = Matrix4::translation(1.0, 2.0, 3.0);
    let translation_3 = Matrix4::translation(1.0, 2.0, 3.0);

    assert!(identity == translation_1);      // no translation equals identity
    assert!(identity != translation_2);      // translation not equals identity
    assert!(translation_2 == translation_3); // same translations are equal

    // check values directly
    assert!(translation_2[0][3] == 1.0);
    assert!(translation_2[1][3] == 2.0);
    assert!(translation_2[2][3] == 3.0);
    assert!(translation_2[3][3] == 1.0);
}
