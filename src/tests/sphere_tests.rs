use crate::{matrix::Matrix, sphere::Sphere};

#[test]
fn testSphereDefaultTransform() {
    let s = Sphere::new();
    assert_eq!(s.transform, Matrix::identity_matrix(4))
}

#[test]
fn testSetTransform() {
    let mut s = Sphere::new();
    s.set_transform(Matrix::identity_matrix(4).scale(2.3, 5.1, 1.1));
    assert_eq!(s.transform, Matrix::identity_matrix(4).scale(2.3, 5.1, 1.1));
}
