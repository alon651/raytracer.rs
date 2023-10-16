use crate::intersections::{Intersectable, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

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

    fn intersect_withoutTRansofrmation(&self, ray: &Ray) -> Intersections {
        match self{
            Object::Sphere(ref s)=>s.intersect_withoutTRansofrmation(ray)
        }
    }

    fn get_material(&self) -> &Material {
        match self {
            Object::Sphere(ref s)=>s.get_material()
        }
    }

     fn set_transform(&mut self, t: Matrix) {
         match self {
             Object::Sphere(ref mut s)=>s.set_transform(t)
         }
     }

     fn normal_at(&self, point: Tuple) -> Tuple {
         match self {
             Object::Sphere(ref s)=>s.normal_at(point)
         }
     }
 }