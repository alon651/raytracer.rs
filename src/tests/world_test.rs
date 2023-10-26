use crate::color::Color;
use crate::intersections::{Intersectable, Intersection, Intersections};
use crate::light::Light;
use crate::matrix::Matrix;
use crate::object::Object::Sphere;
use crate::ray::Ray;
use crate::sphere;
use crate::tuple::Tuple;
use crate::utils::prepare_computations;
use crate::world::World;

fn default_world() -> World {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
    World {
        objects: vec![Sphere(s1), Sphere(s2)],
        lights: vec![l1],
    }
}
#[test]
fn creating_a_world() {
    let w = World::default();
    assert_eq!(w.lights.len(), 0);
    assert_eq!(w.objects.len(), 0);
}
#[test]
fn intersect_a_world() {
    let w = default_world();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let xs: Intersections = w.intersect(&r);
    xs.intersections.iter().for_each(|x| println!("{}", x.time));
    assert_eq!(xs.len(), 4);
}
#[test]
fn precomputing() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
    let shape = Sphere(sphere::Sphere::new());
    let i = Intersection {
        time: 4.0,
        object_ref: Box::new(shape),
    };
    let p = prepare_computations(&i, r, &Intersections::new(vec![i.clone()]));
    assert_eq!(i.time, p.t);
    assert_eq!(p.point, Tuple::new_point(0.0, 0.0, -1.0));
    assert_eq!(p.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(p.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
}

#[test]
fn ray_misses() {
    let w = default_world();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 1., 0.));
    let c = w.color_at(r, 1);
    assert_eq!(c, Color::new(0., 0., 0.));
}

#[test]
fn ray_hits() {
    let w = default_world();
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
    let c = w.color_at(r, 1);
    assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594));
}
#[test]
fn intersection_behind_ray() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.ambient = 1.0;
    let mut s2 = sphere::Sphere::new();
    s2.material.ambient = 1.;
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
    let c2 = s2.material.color;
    let w = World {
        objects: vec![Sphere(s1), Sphere(s2)],
        lights: vec![l1],
    };
    let r = Ray::new(
        Tuple::new_point(0., 0., 0.75),
        Tuple::new_vector(0., 0., -1.),
    );
    let c = w.color_at(r, 1);
    debug_assert_eq!(c, c2);
}

#[test]
pub fn shadow_object_between_point_and_light() {
    let w = default_world();
    let point = Tuple::new_point(10., -10., 10.);
    assert_eq!(w.is_shadow(point, &w.lights[0]), true);
}
#[test]
pub fn shadow_object_behind_the_light() {
    let w = default_world();
    let point = Tuple::new_point(-20., 20., -20.);
    assert_eq!(w.is_shadow(point, &w.lights[0]), false);
}
#[test]
pub fn shadow_object_behind_the_point() {
    let w = default_world();
    let point = Tuple::new_point(-2., 2., -2.);
    assert_eq!(w.is_shadow(point, &w.lights[0]), false);
}

#[test]
pub fn intersection_in_shadow() {
    let mut w = World::default();
    w.lights.push(Light::new(
        Color::new(1., 1., 1.),
        Tuple::new_point(0., -0., -10.),
    ));
    let s1 = sphere::Sphere::new();
    w.push_obj(Sphere(s1));
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).translation(0., 0., 10.));
    w.push_obj(Sphere(s2.clone()));
    let r = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.));
    // let i = Intersection{
    //     object_ref: Box::new(Sphere(s2)),
    //     time: 4.,
    // };
    let binding = w.intersect(&r);
    let i = binding.hits().unwrap();
    println!("{i:?}");
    let comps = prepare_computations(i, r, &Intersections::new(vec![i.clone()]));
    let c = w.shade_hit(comps, 1);
    println!("{c:?}")
}
