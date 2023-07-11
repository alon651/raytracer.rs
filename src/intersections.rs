use crate::ray::Ray;

pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

pub struct Intersection {
    pub objectId: usize,
    pub time: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersections;
    // Other methods specific to intersectable objects
}

impl<'a> Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }
    pub fn len(&self) -> usize {
        self.intersections.len()
    }
}
impl<'a> std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}
