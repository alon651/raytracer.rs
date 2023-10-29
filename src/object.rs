use crate::color::Color;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::patterns::Pattern;
use crate::ray::Ray;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use std::rc::Rc;
use crate::shapes::cube::Cube;

#[derive(PartialEq, Debug, Clone)]
pub struct Object {
    shape: Shape,
    pub transform: Matrix,
    pub material: Material,
    pub inverse: Matrix,
}

impl Object {
    pub(crate) fn local_intersect(&self, ray: &Ray) -> Intersections {
        let v = self.shape.intersect(ray);
        let mut res = Intersections::new(vec![]);
        for i in v {
            res.intersections.push(Intersection {
                object_ref: Rc::new(self.clone()),
                time: i,
            })
        }
        res
    }

    pub(crate) fn set_transform(&mut self, t: Matrix) {
        self.transform = t.clone();
        self.inverse = t.inverse();
    }

    pub(crate) fn normal_at(&self, point: Tuple) -> Tuple {
        let point = &self.inverse * point;
        let mut w = &self.inverse.transpose() * self.shape.normal_at(point);
        w.w = 0.;
        w.normalize()
    }
}
impl Object {
    pub fn stripe_at_object(&self, pattern: &Pattern, world_point: Tuple) -> Color {
        let object_point = &self.inverse * world_point;
        let pattern_point = &pattern.inverse_transform * object_point;
        pattern.pattern_at(pattern_point)
    }
}

impl Object{
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity_matrix(4).inverse(),
            material: Default::default(),
            inverse: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane(Plane::new()),
            transform: Matrix::identity_matrix(4).inverse(),
            material: Default::default(),
            inverse: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn new_cube() -> Self {
        Object {
            shape: Shape::Cube(Cube::new()),
            transform: Matrix::identity_matrix(4).inverse(),
            material: Default::default(),
            inverse: Matrix::identity_matrix(4).inverse(),
        }
    }
    ///resize the object by x,y,z units in the x,y,z axis
    pub fn resize(&mut self,x:f32, y:f32, z:f32){
        self.set_transform(&self.transform*&Matrix::identity_matrix(4).scale(x,y,z));
    }
    ///rotate the object by x,y,z units around the x,y,z axis
    pub fn rotate(&mut self,x:f32, y:f32, z:f32){
        self.set_transform(&self.transform*&Matrix::identity_matrix(4).rotate_x(x));
        self.set_transform(&self.transform*&Matrix::identity_matrix(4).rotate_y(y));
        self.set_transform(&self.transform*&Matrix::identity_matrix(4).rotate_z(z));
    }
    ///move the object by x,y,z units in the x,y,z axis
    pub fn move_by(&mut self, x:f32, y:f32, z:f32){
        self.set_transform(&self.transform*&Matrix::identity_matrix(4).translation(x,y,z));
    }
}