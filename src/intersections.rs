use std::cmp::Ordering;
use crate::{matrix::Matrix, ray::Ray};
use crate::material::Material;
use crate::object::Object;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Intersection{
    pub object_ref: Box<Object>,
    pub time: f32,
}

pub trait Intersectable {
    fn get_transform(&self) -> &Matrix;
    ///this function doesn't apply transformation, please use ray.intersect() instead
    fn intersect_withoutTRansofrmation(&self, ray: &Ray) -> Intersections;
    fn get_material(&self)->&Material;
    fn set_transform(&mut self, t: Matrix);
    fn normal_at(&self, point: Tuple) -> Tuple;
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Self { intersections }
    }
    pub fn len(&self) -> usize {
        self.intersections.len()
    }
    pub fn hits(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|item| item.time >= 0.0)
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
    }
}
impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl std::ops::Add<Intersections> for Intersections{
    type Output = Intersections;

    fn add(self, rhs: Intersections) -> Self::Output {
        Intersections{ intersections: [self.intersections, rhs.intersections].concat() }
    }
}
