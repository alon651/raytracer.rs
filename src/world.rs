use crate::color::Color;
use crate::intersections::{Intersectable, Intersection, Intersections};
use crate::light::Light;
use crate::material::lighting;
use crate::object::Object;
use crate::ray::Ray;
use crate::utils::{precomp, prepare_computations};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub(crate) fn intersect(&self, ray: &Ray) -> Intersections {
        let mut res = Intersections::new(vec![]);
        for obj in &self.objects {
            res = ray.intersect(&obj)+res;
        }
        res.intersections
            .sort_by(|x, y| x.time.partial_cmp(&y.time).unwrap());
        res
    }
    pub fn shade_hit(&self,comps:precomp)->Color{
        let mut c = Color::new(0., 0., 0.);
        self.lights.iter().for_each(|light|{c=c+lighting(comps.obj_ref.get_material(),light,comps.point,comps.eyev,comps.normalv)});
        c
    }
    pub fn color_at(&self,r:Ray)->Color{
        let xs = self.intersect(&r);
        if let Some(hit) = xs.hits(){
            let comps = prepare_computations(xs.hits().unwrap(),r);
            return self.shade_hit(comps);
        };
        Color::new(0.0,0.0,0.0)
    }
}
