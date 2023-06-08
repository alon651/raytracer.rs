use crate::ray::{self, Ray};
use crate::sphere::Sphere;
use crate::{tuple::Tuple, utils::cmp_f32};

#[test]
fn test_ray() {
    let origin = Tuple::new_point(1.0, 2.0, 3.0);
    let direction = Tuple::new_vector(4.0, 5.0, 6.0);
    let r = Ray { origin, direction };
    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}
#[test]
fn compute_point_from_distance() {
    let origin = Tuple::new_point(2.0, 3.0, 4.0);
    let direction = Tuple::new_vector(1.0, 0.0, 0.0);
    let r = Ray { origin, direction };
    assert_eq!(r.position(0.0), origin);
    assert_eq!(r.position(1.0), Tuple::new_point(3.0, 3.0, 4.0));
    assert_eq!(r.position(2.0), Tuple::new_point(4.0, 3.0, 4.0));
}
#[test]
fn test_intersections() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 0.0, -5.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}
#[test]
fn test_intersect_at_a_tangent() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 1.0, -5.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}
#[test]
fn ray_missing_the_sphere() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 2.0, -5.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 0);
}
#[test]
fn test_intersections_from_behind() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 0.0, 0.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = r.intersect(s);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}
