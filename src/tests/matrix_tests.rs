use std::f32::consts::SQRT_2;

use crate::{matrix::Matrix, tuple::Tuple};
#[test]
pub fn create_and_get_4x4_matrix_values() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let m = Matrix::new(4, 4, data).unwrap();
    assert_eq!(m[3][2], 15.5);
}
#[test]
pub fn create_2_matrixes_and_compare_them() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let m = Matrix::new(4, 4, data).unwrap();

    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let m2 = Matrix::new(4, 4, data).unwrap();
    assert_eq!(m2, m);
}
#[test]
pub fn create_2_matrixes_and_multiply_them() {
    //create a vector with all numbers from 0 to 15
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let mut data2 = data.clone();
    let m = Matrix::new(4, 4, data).unwrap();
    data2.reverse();
    let m1 = Matrix::new(4, 4, data2).unwrap();

    let m = &m * &m1;
    assert_eq!(
        m,
        Matrix::new(
            4,
            4,
            vec![
                80.0, 70.0, 60.0, 50.0, 240.0, 214.0, 188.0, 162.0, 400.0, 358.0, 316.0, 274.0,
                560.0, 502.0, 444.0, 386.0
            ]
        )
        .unwrap()
    );
}

#[test]
fn make_matrix_and_multiply_with_tuple() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let m = Matrix::new(4, 4, data).unwrap();
    let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(&m * t, Tuple::new(30.0, 70.0, 110.0, 150.0));
}

#[test]
fn matrix_multiplied_by_identity() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let m = Matrix::new(4, 4, data).unwrap();
    assert_eq!(&m * &Matrix::identity_matrix(4), m)
}

#[test]
fn tuple_multiplied_by_identity() {
    let t = Tuple::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(&Matrix::identity_matrix(4) * t, t)
}

#[test]
fn check_if_transpose_is_correct() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let m = Matrix::new(4, 4, data).unwrap();
    assert_eq!(
        m.transpose(),
        Matrix::new(
            4,
            4,
            vec![
                1.0, 5.0, 9.0, 13.0, 2.0, 6.0, 10.0, 14.0, 3.0, 7.0, 11.0, 15.0, 4.0, 8.0, 12.0,
                16.0,
            ]
        )
        .unwrap()
    )
}

#[test]
fn check_2x2_det() {
    let data = vec![1.0, 5.0, -3.0, 2.0];
    assert_eq!(Matrix::new(2, 2, data).unwrap().determinant(), 17.0);
}

#[test]
fn check_submatrix() {
    let m = Matrix::new(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0])
        .unwrap()
        .submatrix(0, 2);
    println!("{m:?}");
    assert_eq!(1, 1);
}

#[test]
fn check_minor() {
    assert_eq!(
        Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0])
            .unwrap()
            .minor(1, 0),
        25.0
    );
}

#[test]
fn determinant_of_large_matrix() {
    let m = Matrix::new(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]).unwrap();
    assert_eq!(m.cofactor(0, 0), 56.0);
    assert_eq!(m.cofactor(0, 1), 12.0);
    assert_eq!(m.cofactor(0, 2), -46.0);
    assert_eq!(m.determinant(), -196.0);
}

#[test]
fn test_invert() {
    let data = vec![
        -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
    ];
    let m = Matrix::new(4, 4, data).unwrap().inverse();
    println!("{m:?}");
    assert_eq!(
        m,
        Matrix {
            data: vec![
                0.21804512,
                0.45112783,
                0.24060151,
                -0.04511278,
                -0.8082707,
                -1.456767,
                -0.44360903,
                0.5206767,
                -0.078947365,
                -0.2236842,
                -0.05263158,
                0.19736843,
                -0.52255636,
                -0.81390977,
                -0.30075186,
                0.30639097
            ],
            n_rows: 4,
            n_cols: 4
        }
    );
}

#[test]
fn matrix_multiply_inverse() {
    let data = vec![
        -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
    ];
    let m = Matrix::new(4, 4, data).unwrap();
    let m2 = m.inverse();
    assert_eq!(&m * &m2, Matrix::identity_matrix(4));
}

#[test]
fn translation_matrix() {
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let translation = Matrix::translation(5.0, -3.0, 2.0);
    let p = &translation * point;
    assert_eq!(p, Tuple::new_point(7.0, 0.0, 6.0));
}

#[test]
pub fn inverse_translation_matrix() {
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let translation = Matrix::translation(5.0, -3.0, 2.0).inverse();
    let p = &translation * point;
    assert_eq!(p, Tuple::new_point(-3.0, 6.0, 2.0));
}

#[test]
fn translate_vector() {
    let v = Tuple::new_vector(2.0, 3.0, 4.0);
    let translation = Matrix::translation(5.0, -3.0, 2.0);
    assert_eq!(&translation * v, v);
}

#[test]
fn scaling_matrix() {
    let transform = Matrix::scale(2.0, 3.0, 4.0);
    let v = Tuple::new_vector(-4.0, 6.0, 8.0);
    assert_eq!(&transform * v, Tuple::new_vector(-8.0, 18.0, 32.0));
}

#[test]
fn rotate() {
    let p = Tuple::new_point(0.0, 1.0, 0.0);

    assert_eq!(
        &Matrix::rotate_x((std::f64::consts::FRAC_PI_4 as f32).into()).inverse() * p,
        Tuple::new_point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0)
    );
    let p = Tuple::new_point(0.0, 0.0, 1.0);
    assert_eq!(
        &Matrix::rotate_y((std::f64::consts::FRAC_PI_4 as f32).into()) * p,
        Tuple::new_point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0)
    );
    let p = Tuple::new_point(0.0, 1.0, 0.0);
    assert_eq!(
        &Matrix::rotate_z((std::f64::consts::FRAC_PI_4 as f32).into()) * p,
        Tuple::new_point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0)
    );
}
