use crate::intersections::{Intersectable, Intersections};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
#[derive(PartialEq,Debug,Clone)]
pub enum Object{
    Sphere(Sphere)
}

impl Intersectable for Object {
    fn get_transform(&self) -> &Matrix {
        match self{
            Object::Sphere(ref s)=>s.get_transform()
        }
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        match self{
            Object::Sphere(ref s)=>s.intersect(ray)
        }
    }
}