use crate::matrix::Matrix;
#[test]
pub fn create_and_get_4x4_matrix_values() {
    let data = vec![
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ];
    let mut m = Matrix::new(4, 4, data).unwrap();
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            print!("{} ", m.get(y, x).unwrap());
            x += 1;
        }
        println!();
        y += 1;
    }
    assert_eq!(m.get(3, 2).unwrap(), 15.5);
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
    let mut m2 = Matrix::new(4, 4, data).unwrap();
    assert_eq!(m2, m);
}
