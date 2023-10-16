use std::sync::atomic::{AtomicUsize, Ordering};
use crate::intersections::{Intersectable, Intersection};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

const EPSILON: f32 = 0.0001;
pub fn cmp_f32(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}

static IDS: AtomicUsize = AtomicUsize::new(0);

pub fn generate_id() -> usize {
    IDS.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug)]
pub struct precomp{
    pub t:f32,
    pub obj_ref: Box<Object>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside:bool,
}

pub fn prepare_computations(intersection: &Intersection, ray:Ray) ->precomp{

     let mut p = precomp{
        t:intersection.time,
        point: ray.position(intersection.time),
        eyev: -ray.direction,
        normalv:intersection.object_ref.normal_at(ray.position(intersection.time)),
        obj_ref:intersection.object_ref.clone(),
        inside: false
    };
    if p.normalv * p.eyev < 0.0 {
        p.inside = true;
        p.normalv = -p.normalv;
    }
    p
}