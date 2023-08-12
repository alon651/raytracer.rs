use crate::intersections::{self, Intersectable, Intersection, Intersections};
use crate::matrix::Matrix;
use crate::ray::{self, Ray};
use crate::sphere::Sphere;
use crate::{tuple::Tuple, utils::cmp_f32};
use std::any::Any;

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
    let xs = {
        let ref this = r;
        let other = &s;
        other.intersect(this)
    };
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].time, 4.0);
    assert_eq!(xs[1].time, 6.0);
}
#[test]
fn test_intersect_at_a_tangent() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 1.0, -5.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = {
        let ref this = r;
        let other = &s;
        other.intersect(this)
    };
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].time, 5.0);
    assert_eq!(xs[1].time, 5.0);
}
#[test]
fn ray_missing_the_sphere() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 2.0, -5.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = {
        let ref this = r;
        let other = &s;
        other.intersect(this)
    };
    assert_eq!(xs.len(), 0);
}
#[test]
fn test_intersections_from_behind() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 0.0, 0.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = {
        let ref this = r;
        let other = &s;
        other.intersect(this)
    };
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].time, -1.0);
    assert_eq!(xs[1].time, 1.0);
}

#[test]
fn test_object_tracking() {
    let r = Ray {
        origin: Tuple::new_point(0.0, 0.0, 0.0),
        direction: Tuple::new_vector(0.0, 0.0, 1.0),
    };
    let s = Sphere::new();
    let xs = {
        let ref this = r;
        let other = &s;
        other.intersect(this)
    };

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object_id, s.id);
    assert_eq!(xs[1].time, 1.0);
}

#[test]
fn test_unique_id() {
    let s = Sphere::new();
    let s1 = Sphere::new();
    assert_ne!(s.id, s1.id);
}

#[test]
fn testHItsFunction() {
    let i1 = Intersection {
        object_id: 1,
        time: 7.0,
    };

    let i2 = Intersection {
        object_id: 1,
        time: -3.0,
    };
    let i3 = Intersection {
        object_id: 1,
        time: 2.0,
    };
    let i4 = Intersection {
        object_id: 1,
        time: 5.0,
    };
    let xs = Intersections::new(vec![i1, i2, i3, i4]);
    assert_eq!(xs.hits().unwrap(), &i3);
}

#[test]
fn testTransformRay() {
    let r = Ray {
        origin: Tuple::new_point(1.0, 2.0, 3.0),
        direction: Tuple::new_vector(0.0, 1.0, 0.0),
    };
    let m = Matrix::identity_matrix(4).translation(3.0, 4.0, 5.0);
    let res = r.transform(m);
    assert_eq!(res.origin, Tuple::new_point(4.0, 6.0, 8.0));
    assert_eq!(res.direction, Tuple::new_vector(0.0, 1.0, 0.0));
}

#[test]
fn testTransformRayScale() {
    let r = Ray {
        origin: Tuple::new_point(1.0, 2.0, 3.0),
        direction: Tuple::new_vector(0.0, 1.0, 0.0),
    };
    let m = Matrix::identity_matrix(4).scale(2.0, 3.0, 4.0);
    let res = r.transform(m);
    assert_eq!(res.origin, Tuple::new_point(2.0, 6.0, 12.0));
    assert_eq!(res.direction, Tuple::new_vector(0.0, 3.0, 0.0));
}
