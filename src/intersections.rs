use crate::{matrix::Matrix, ray::Ray};
use crate::object::Object;

#[derive(Debug)]
pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub object_ref: &'a Object,
    pub time: f32,
}

pub trait Intersectable {
    fn get_transform(&self) -> &Matrix;
    fn intersect(&self, ray: &Ray) -> Intersections;
    // Other methods specific to intersectable objects
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Intersections<'a> {
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
impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}
