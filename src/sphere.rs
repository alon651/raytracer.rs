use std::vec;

use crate::{
    intersections::{Intersectable, Intersection, Intersections},
    material::Material,
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
    utils::generate_id,
};
use crate::object::Object;

#[derive(PartialEq, Debug,Clone)]
pub struct Sphere {
    radius: f32,
    center: Tuple,
    pub id: usize,
    pub transform: Matrix,
    pub material: Material,
}
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            radius: 1.0,
            center: Tuple::new_point(0.0, 0.0, 0.0),
            id: generate_id(),
            transform: Matrix::identity_matrix(4),
            material: Material::default(),
        }
    }
    pub fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        // (point - Tuple::new_point(0.0, 0.0, 0.0)).normalize()
        let object_point = &self.transform.inverse() * point;
        let _object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
        let mut world_normal = &self.transform.transpose().inverse() * object_point;
        world_normal.w = 0.0; //zero the w of the normal to negate size bugs(not elegant but does the job)
        world_normal.normalize()
    }
}
impl Intersectable for Sphere {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn intersect(&self, other: &Ray) -> Intersections {
        let sphere_to_ray = other.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = other.direction * other.direction;
        let b = 2.0 * (other.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new(Vec::new());
        }
        Intersections::new(vec![
            Intersection {
                object_ref: &self.clone(),
                time: (-b - discriminant.sqrt()) / (2.0 * a),
            },
            Intersection {
                object_ref: &Object::Sphere(self.clone()),
                time: (-b + discriminant.sqrt()) / (2.0 * a),
            },
        ])
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
