use crate::intersections::{Intersectable, Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::utils::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    pub material: Material,
    pub transform: Matrix,
}

impl Intersectable for Plane {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn local_intersect(&self, ray: &Ray) -> Intersections {
        if (ray.direction.y).abs() < EPSILON {
            return Intersections::new(vec![]);
        }
        let t = -ray.origin.y / ray.direction.y;
        Intersections::new(vec![Intersection {
            object_ref: Box::new((Object::Plane(self.clone()))),
            time: t,
        }])
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }

    fn normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::new_vector(0., 1., 0.)
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            transform: Matrix::identity_matrix(4),
            material: Material::default(),
        }
    }
}
