use std::cmp::Ordering::Less;
use crate::color::Color;
use crate::intersections::Intersections;
use crate::light::Light;
use crate::material::lighting;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::utils::{prepare_computations, Precomp};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let mut res = Intersections::new(vec![]);
        for obj in &self.objects {
            res += ray.intersect(obj);
        }
        res.intersections
            .sort_by(|x, y| x.time.partial_cmp(&y.time).unwrap_or(Less));
        res
    }
    pub fn shade_hit(&self, comps: Precomp, remaining: usize) -> Color {
        let mut surface = Color::new(0., 0., 0.);
        self.lights.iter().for_each(|light| {
            let is_shadow = self.is_shadow(comps.over_point, light);
            surface = lighting(
                &comps.obj_ref.material,
                &comps.obj_ref,
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                is_shadow,
            )
        });
        let reflected = self.reflected_color(comps.clone(), remaining);
        let refracted = self.refracted_color(comps.clone(), remaining);
        let material = &comps.obj_ref.material;
        if material.reflective > 0. && material.transparency > 0. {
            let reflectance = World::schlick(comps);
            return surface + reflected * reflectance + refracted * (1. - reflectance);
        }
        surface + reflected + refracted
    }
    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let xs = self.intersect(&r);
        if let Some(hit) = xs.hits() {
            let comps = prepare_computations(hit, r, &xs);
            return self.shade_hit(comps, remaining);
        };
        // Color::new(0.529, 0.807, 0.921) * 0.5  //sky blue
        Color::new(0., 0., 0.)
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
            if hit.time < dist /*&& hit.object_ref.material.transparency !=1.*/ {
                return true;
            }
        }
        false
    }
    pub fn reflected_color(&self, comps: Precomp, remaining: usize) -> Color {
        if comps.obj_ref.material.reflective == 0. || remaining <= 0 {
            return Color::new(0., 0., 0.);
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray, remaining - 1);
        color * comps.obj_ref.material.reflective
    }
    pub fn refracted_color(&self, comps: Precomp, remaining: usize) -> Color {
        if comps.obj_ref.material.transparency == 0. || remaining == 0 {
            return Color::new(0., 0., 0.);
        }
        //calculate angel
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev * comps.normalv;
        let sin2_t = n_ratio * n_ratio * (1. - (cos_i * cos_i));
        // if sin2_t > 1. {
        //     return Color::new(0.,0.,0.);
        // }
        let cos_t = (1. - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        let color = self.color_at(refract_ray, remaining - 1) * comps.obj_ref.material.transparency;
        color
    }

    pub fn schlick(comps: Precomp) -> f32 {
        let mut cos = comps.eyev * comps.normalv;
        if comps.n1 > comps.n2 {
            let n = comps.n1 / comps.n2;
            let sin2_t = n * n * (1. - cos * cos);
            if sin2_t > 1. {
                return 1.0;
            }
            let cos_t = (1. - sin2_t).sqrt();
            cos = cos_t;
        };
        let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}
