use crate::ray::Ray;

pub trait Intersect {
    fn calculate_intersection(&self, other: &Ray) -> Vec<f32>;
}
