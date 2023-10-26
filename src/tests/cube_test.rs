use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[test]
pub fn test_intersections(){
    let origin = Tuple::new_point(0.5,5.,0.);
    let direction = Tuple::new_vector(0.,-1.,0.);


    let c = Object::new_cube();
    let r = Ray::new(origin,direction);
    let xs = r.intersect(&c);
    xs.intersections.iter().for_each(|intersection|{
        println!("{}",intersection.time)
    })
}

#[test]
pub fn test_normal(){
    let point = Tuple::new_point(-0.4,1.,-0.1);

    let c = Object::new_cube();
    let normal = c.normal_at(point);
    println!("{:?}", normal)
}