use crate::intersections::{Intersectable, Intersection, Intersections};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const EPSILON: f32 = 0.0012;
pub fn cmp_f32(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}

static IDS: AtomicUsize = AtomicUsize::new(0);

pub fn generate_id() -> usize {
    IDS.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Clone)]
pub struct Precomp {
    pub t: f32,
    pub obj_ref: Box<Object>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflectv: Tuple,
    pub n1: f32,
    pub n2: f32,
    pub under_point: Tuple,
}

pub fn prepare_computations(intersection: &Intersection, ray: Ray, xs: &Intersections) -> Precomp {
    let mut p = Precomp {
        t: intersection.time,
        point: ray.position(intersection.time),
        eyev: -ray.direction,
        normalv: intersection
            .object_ref
            .normal_at(ray.position(intersection.time)),
        obj_ref: intersection.object_ref.clone(),
        inside: false,
        over_point: Tuple::default(),
        reflectv: Tuple::default(),
        n1: 1.0,
        n2: 1.0,
        under_point: Tuple::default(),
    };
    if p.normalv * p.eyev < 0.0 {
        p.inside = true;
        p.normalv = -p.normalv;
    }
    p.over_point = p.point + p.normalv * EPSILON;
    p.under_point = p.point - p.normalv * EPSILON;
    p.reflectv = ray.direction.reflect(p.normalv);

    let mut containers: Vec<&Object> = vec![];
    let hit = intersection;

    for i in &xs.intersections {
        if i == hit {
            if let Some(obj) = containers.last() {
                p.n1 = obj.get_material().refractive_index;
            }
        }
        if containers.contains(&i.object_ref.as_ref()) {
            containers.remove(
                containers
                    .iter()
                    .position(|obj| *obj == i.object_ref.as_ref())
                    .unwrap(),
            );
        } else {
            containers.push(i.object_ref.as_ref())
        }
        if i == hit {
            if let Some(obj) = containers.last() {
                p.n2 = obj.get_material().refractive_index;
            }
        }
    }
    p
}
