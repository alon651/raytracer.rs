use crate::intersections::Intersectable;
use crate::material::Material;
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

// #[test]
// fn testNewIntersectNoHits() {
//     let r = Ray::new(
//         Tuple::new_point(0.0, 0.0, -5.0),
//         Tuple::new_vector(0.0, 0.0, 1.0),
//     );
//     let mut s = Sphere::new();
//     s.set_transform(Matrix::identity_matrix(4).translation(5.0, 0.0, 0.0));
//     let xs = r.intersect(&s);
//     assert_eq!(xs.len(), 0)
// }

// #[test]
// fn testNewIntersectWithHits() {
//     let r = Ray::new(
//         Tuple::new_point(0.0, 0.0, -5.0),
//         Tuple::new_vector(0.0, 0.0, 1.0),
//     );
//     let mut s = Sphere::new();
//     s.set_transform(Matrix::identity_matrix(4).scale(2.0, 2.0, 2.0));
//     let xs = r.intersect(&s);
//     assert_eq!(xs.len(), 2);
//     assert_eq!(xs[0].time, 3.0);
//     assert_eq!(xs[1].time, 7.0);
// }
#[test]
fn x_axis_normal() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(1.0, 0.0, 0.0));
    assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
}
#[test]
fn y_axis_normal() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(0.0, 1.0, 0.0));
    assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
}
#[test]
fn z_axis_normal() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(0.0, 0.0, 1.0));
    assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
}
#[test]

fn not_axis_normal() {
    let sqrt3 = f32::sqrt(3.0) / 3.0;
    let s = Sphere::new();
    let n = s.normal_at(Tuple::new_point(sqrt3, sqrt3, sqrt3));
    assert_eq!(n, Tuple::new_vector(sqrt3, sqrt3, sqrt3));
    assert_eq!(n, n.normalize());
}

#[test]
fn test_normal_of_transformed_sphere() {
    let mut s = Sphere::new();
    s.set_transform(Matrix::identity_matrix(4).translation(0.0, 1.0, 0.0));
    let n = s.normal_at(Tuple::new_point(0.0, 1.7, -0.7));
    assert_eq!(n, Tuple::new_vector(0.0, 0.7071068, -0.70710677))
}

#[test]
fn sphere_material() {
    let s = Sphere::new();
    let m = Material::default();
    assert_eq!(s.material, m)
}

#[test]
fn sphere_material_change() {
    let mut s = Sphere::new();
    let mut m = Material::default();
    m.ambient = 1.0;
    s.material = m.clone();
    assert_eq!(s.material, m)
}
