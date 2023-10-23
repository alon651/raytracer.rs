use crate::color::Color;
use crate::intersections::{Intersectable, Intersections};
use crate::light::Light;
use crate::material::lighting;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::utils::{Precomp, prepare_computations};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let mut res = Intersections::new(vec![]);
        for obj in &self.objects {
            res = ray.intersect(obj) + res;
        }
        res.intersections
            .sort_by(|x, y| x.time.partial_cmp(&y.time).unwrap());
        res
    }
    pub fn shade_hit(&self, comps: Precomp) -> Color {
        let mut c = Color::new(0., 0., 0.);
        self.lights.iter().for_each(|light| {
            let is_shadow = self.is_shadow(comps.over_point, light);
            c = lighting(
                comps.obj_ref.get_material(),
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                is_shadow,
            )
        });
        c
    }
    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(&r);
        if let Some(hit) = xs.hits() {
            let comps = prepare_computations(hit, r);
            return self.shade_hit(comps);
        };
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn push_obj(&mut self, obj: Object) {
        self.objects.push(obj);
    }
    pub fn is_shadow(&self, point: Tuple, l: &Light) -> bool {
        let v = l.position - point;
        let dist = v.magnitude();
        let dir = v.normalize();

        let r = Ray::new(point, dir);
        let intersection = self.intersect(&r);
        if let Some(hit) = intersection.hits() {
            if hit.time < dist {
                return true;
            }
        }
        false
    }
}
