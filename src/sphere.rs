use std::vec;

use crate::object::Object;
use crate::{
    intersections::{Intersectable, Intersection, Intersections},
    material::Material,
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
    utils::generate_id,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
    radius: f32,
    center: Tuple,
    pub id: usize,
    transform: Matrix,
    pub material: Material,
    pub inverse: Matrix,
}
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            radius: 1.0,
            center: Tuple::new_point(0.0, 0.0, 0.0),
            id: generate_id(),
            transform: Matrix::identity_matrix(4),
            material: Material::default(),
            inverse: Matrix::identity_matrix(4).inverse()
        }
    }
}
impl Intersectable for Sphere {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn get_inverse(&self) -> &Matrix {
        &self.inverse
    }

    fn local_intersect(&self, other: &Ray) -> Intersections {
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
                object_ref: Box::new(Object::Sphere(self.clone())),
                time: (-b - discriminant.sqrt()) / (2.0 * a),
            },
            Intersection {
                object_ref: Box::new(Object::Sphere(self.clone())),
                time: (-b + discriminant.sqrt()) / (2.0 * a),
            },
        ])
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
    fn set_transform(&mut self, t: Matrix) {
        self.transform = t.clone();
        self.inverse = t.inverse();
    }
    fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = &self.inverse * point;
        let object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
        object_normal.normalize()
        // let mut world_normal = &self.transform.transpose().inverse() * object_point;
        // world_normal.w = 0.0; //zero the w of the normal to negate size bugs(not elegant but does the job)
        // world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
