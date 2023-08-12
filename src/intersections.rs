use crate::{matrix::Matrix, ray::Ray};

pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Intersection {
    pub object_id: usize,
    pub time: f32,
}

pub trait Intersectable {
    fn getTransform(&self) -> Matrix;
    fn intersect(&self, ray: &Ray) -> Intersections;
    // Other methods specific to intersectable objects
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }
    pub fn len(&self) -> usize {
        self.intersections.len()
    }
}
impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl Intersections {
    pub fn hits(&self) -> &Intersection {
        self.intersections
            .iter()
            .filter(|item| item.time >= 0.0)
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
            .unwrap()
    }
}
