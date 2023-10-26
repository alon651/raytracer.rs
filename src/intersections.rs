use std::rc::Rc;
use crate::object::Object;


#[derive(Debug)]
pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Intersection {
    pub object_ref: Rc<Object>,
    pub time: f32,
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

impl std::ops::Add<Intersections> for Intersections {
    type Output = Intersections;

    fn add(self, rhs: Intersections) -> Self::Output {
        Intersections {
            intersections: [self.intersections, rhs.intersections].concat(),
        }
    }
}
