use crate::{tuple::Tuple, utils::cmp_f32};

#[test]
pub fn points_test() {
    let point = Tuple::new_point(4.0, -4.0, 3.0);

    assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
pub fn vectors_test() {
    let vector = Tuple::new_vector(4.0, -4.0, 3.0);
    assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
pub fn tuples_addition() {
    let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0))
}

#[test]
pub fn points_substraction() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::new_vector(-2.0, -4.0, -6.0))
}

#[test]
pub fn vector_from_point_substraction() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_vector(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::new_point(-2.0, -4.0, -6.0))
}
#[test]
pub fn vectors_substraction() {
    let p1 = Tuple::new_vector(3.0, 2.0, 1.0);
    let p2 = Tuple::new_vector(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::new_vector(-2.0, -4.0, -6.0))
}

#[test]
pub fn substract_vector_from_zero() {
    let v = Tuple::new_vector(1.0, -2.0, 3.0);
    let zero = Tuple::new_vector(0.0, 0.0, 0.0);
    assert_eq!(zero - v, Tuple::new_vector(-1.0, 2.0, -3.0))
}
#[test]
pub fn negate_vector() {
    let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-v, Tuple::new(-1.0, 2.0, -3.0, 4.0))
}

#[test]
pub fn multiply_vector_by_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0))
}
#[test]
pub fn multiply_vector_by_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0))
}

#[test]
pub fn magnitude() {
    let v = Tuple::new_vector(1.0, 2.0, 3.0);

    assert_eq!(v.magnitude(), f32::from(14.0).sqrt())
}
#[test]
pub fn magnitude_of_normal_vector() {
    let v = Tuple::new_vector(1.0, 2.0, 3.0).normalize();
    assert_eq!(cmp_f32(v.magnitude(), 1.0), true);
}

#[test]
pub fn dot_product_of_vectors() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);
    assert_eq!(a * b, 20.0);
}
#[test]
pub fn cross_product() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), Tuple::new_vector(-1.0, 2.0, -1.0));
}

#[test]
pub fn reflecting_vector_at_45() {
    let v = Tuple::new_vector(1.0, -1.0, 0.0);
    let n = Tuple::new_vector(0.0, 1.0, 0.0);
    let r = v.reflect(n);
    assert_eq!(
        r,
        Tuple {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            w: 0.0
        }
    )
}

#[test]
pub fn reflecting_vector_of_slanted() {
    let v = Tuple::new_vector(0.0, -1.0, 0.0);
    let n = Tuple::new_vector(0.707, 0.707, 0.0);
    let r = v.reflect(n);
    assert_eq!(
        r,
        Tuple {
            x: 0.99969804,
            y: -0.00030195713,
            z: 0.0,
            w: 0.0
        }
    );
}
