
use std::cmp::Ordering:: Less;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::utils::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Cube{

}
impl Cube{
    fn check_axis(&self,origin:f32,direction:f32)->(f32,f32){
        let tmin_numerator = -1.-origin;
        let tmax_numerator = 1.- origin;
        let mut tmin = 0.;
        let mut tmax = 0.;
        if direction.abs() >= EPSILON{
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator/direction;
        }else{
            tmin = tmin_numerator * f32::INFINITY;
            tmax = tmax_numerator* f32::INFINITY;
        }
        if tmin > tmax{
            return (tmax,tmin);
        }
        return (tmin,tmax)
    }
    pub fn local_intersect(&self, ray:&Ray) -> Vec<f32> {
        let (xtmin,xtmax) = self.check_axis(ray.origin.x,ray.direction.x);
        let (ytmin,ytmax) = self.check_axis(ray.origin.y,ray.direction.y);
        let (ztmin,ztmax) = self.check_axis(ray.origin.z,ray.direction.z);
        let binding = [xtmin,ytmin,ztmin];

        let tmin = *binding.iter().max_by(|x,y|{
            x.partial_cmp(&y).unwrap_or(Less)
        }).unwrap();
        let binding = [xtmax, ytmax, ztmax];
        let tmax = *binding.iter().min_by(|x, y|{
            x.partial_cmp(&y).unwrap_or(Less)
        }).unwrap();
        if tmin > tmax {
            return vec![]
        }
        vec![tmin,tmax]
    }
    pub(crate) fn normal_at(&self, point:Tuple) ->Tuple{
        let maxc = *[point.x.abs(),point.y.abs(),point.z.abs()].iter().max_by(|x,y|{
            x.partial_cmp(&y).unwrap_or(Less)
        }).unwrap();
        if maxc == point.x.abs() {
            return Tuple::new_vector(point.x,0.,0.)
        }else {
            if maxc == point.y.abs() {
                return Tuple::new_vector(0.,point.y,0.);
            }
        }
        Tuple::new_vector(0.,0.,point.z)
    }
    pub fn new()->Self{
        Cube{}
    }
}