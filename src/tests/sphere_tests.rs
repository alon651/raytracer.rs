use crate::{matrix::Matrix, ray::Ray, sphere::Sphere, tuple::Tuple};

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

#[test]
fn testNewIntersectNoHits() {
    let r = Ray::new(
        Tuple::new_point(0.0, 0.0, -5.0),
        Tuple::new_vector(0.0, 0.0, 1.0),
    );
    let mut s = Sphere::new();
    s.set_transform(Matrix::identity_matrix(4).translation(5.0, 0.0, 0.0));
    let xs = r.intersect(&s);
    assert_eq!(xs.len(), 0)
}

#[test]
fn testNewIntersectWithHits() {
    let r = Ray::new(
        Tuple::new_point(0.0, 0.0, -5.0),
        Tuple::new_vector(0.0, 0.0, 1.0),
    );
    let mut s = Sphere::new();
    s.set_transform(Matrix::identity_matrix(4).scale(2.0, 2.0, 2.0));
    let xs = r.intersect(&s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].time, 3.0);
    assert_eq!(xs[1].time, 7.0);
}
