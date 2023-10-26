use crate::intersections::{Intersections};

use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
}

impl Plane {


    pub(crate) fn local_intersect(&self, ray: &Ray) -> Intersections {
        todo!();
        // if (ray.direction.y).abs() < EPSILON {
        //     return Intersections::new(vec![]);
        // }
        // let t = -ray.origin.y / ray.direction.y;
        // Intersections::new(vec![Intersection {
        //     object_ref: Box::new(()),
        //     time: t,
        // }])
    }


    pub(crate) fn normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::new_vector(0., 1., 0.)
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {

        }
    }
}
