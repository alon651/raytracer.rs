use crate::matrix::Matrix;
#[test]
pub fn create_and_get_4x4_matrix_values() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let mut m = Matrix::new(4, 4, data).unwrap();
    let mut y = 0;
    assert_eq!(m[3][2], 15.5);
}
#[test]
pub fn create_2_matrixes_and_compare_them() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let mut m = Matrix::new(4, 4, data).unwrap();

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

    let m = m * m1;
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
