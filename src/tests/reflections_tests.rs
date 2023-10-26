use crate::color::Color;
use crate::intersections::{Intersectable, Intersection, Intersections};
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::utils::{prepare_computations, EPSILON};
use crate::world::World;
use std::f32::consts::SQRT_2;

// fn default_world() -> World {
//     let mut s1 = sphere::Sphere::new();
//     s1.material.color = Color::new(0.8, 1.0, 0.6);
//     s1.material.diffuse = 0.7;
//     s1.material.specular = 0.2;
//     let mut s2 = sphere::Sphere::new();
//     s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
//     let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
//     World {
//         objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
//         lights: vec![l1],
//     }
// }
#[test]
pub fn default_material_reflectivity() {
    let m = Material::default();
    assert_eq!(m.reflective, 0.0)
}
#[test]
pub fn precompute_reflect_vector() {
    let shape = Plane::new();
    let r = Ray::new(
        Tuple::new_point(0., 1., -1.),
        Tuple::new_vector(0., -0.707, 0.707),
    );
    let i = Intersection {
        object_ref: Box::new(Object::Plane(shape)),
        time: std::f32::consts::SQRT_2,
    };
    let comps = prepare_computations(&i, r, &Intersections::new(vec![i.clone()]));
    assert_eq!(comps.reflectv, Tuple::new_vector(0., 0.707, 0.707))
}

#[test]
pub fn strike_non_reflective() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    s2.material.ambient = 1.;
    let i = Intersection {
        object_ref: Box::new(Object::Sphere(s2.clone())),
        time: 1.0,
    };
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
    let w = World {
        objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
        lights: vec![l1],
    };
    let r = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.));
    let comps = prepare_computations(&i, r, &Intersections::new(vec![i.clone()]));
    let color = w.reflected_color(comps, 1);
    assert_eq!(color, Color::new(0., 0., 0.));
}

#[test]
pub fn strike_reflective_surface() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));

    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.set_transform(Matrix::identity_matrix(4).translation(0., -1., 0.));
    let w = World {
        objects: vec![
            Object::Sphere(s1),
            Object::Sphere(s2),
            Object::Plane(shape.clone()),
        ],
        lights: vec![l1],
    };
    let r = Ray::new(
        Tuple::new_point(0., 0., -3.),
        Tuple::new_vector(0., -0.707, 0.707),
    );
    let i = Intersection {
        object_ref: Box::new(Object::Plane(shape)),
        time: SQRT_2,
    };
    let comps = prepare_computations(&i, r, &Intersections::new(vec![i.clone()]));
    let color = w.reflected_color(comps, 1);
    assert_eq!(color, Color::new(0.1905753, 0.23821911, 0.14293148));
}

#[test]
pub fn shade_hit_strike_reflective_surface() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));

    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.set_transform(Matrix::identity_matrix(4).translation(0., -1., 0.));
    let w = World {
        objects: vec![
            Object::Sphere(s1),
            Object::Sphere(s2),
            Object::Plane(shape.clone()),
        ],
        lights: vec![l1],
    };
    let r = Ray::new(
        Tuple::new_point(0., 0., -3.),
        Tuple::new_vector(0., -0.707, 0.707),
    );
    let i = Intersection {
        object_ref: Box::new(Object::Plane(shape)),
        time: SQRT_2,
    };
    let comps = prepare_computations(&i, r, &Intersections::new(vec![i.clone()]));
    let color = w.shade_hit(comps, 1);
    assert_eq!(color, Color::new(0.8769617, 0.9246055, 0.82931787));
}

//refraction and transparency tests
#[test]
pub fn default_values() {
    let m = Material::default();
    assert_eq!(m.transparency, 0.);
    assert_eq!(m.refractive_index, 1.);
}

pub fn glass_sphere() -> Sphere {
    let mut s = Sphere::new();
    s.material.refractive_index = 1.5;
    s.material.transparency = 1.0;
    s
}

#[test]
pub fn n1andn2() {
    let mut A = glass_sphere();
    A.set_transform(Matrix::identity_matrix(4).scale(2., 2., 2.));
    A.material.refractive_index = 1.5;
    let mut B = glass_sphere();
    B.set_transform(Matrix::identity_matrix(4).translation(0., 0., -0.25));
    B.material.refractive_index = 2.;
    let mut C = glass_sphere();
    C.set_transform(Matrix::identity_matrix(4).translation(0., 0., 0.25));
    C.material.refractive_index = 2.5;
    let r = Ray::new(Tuple::new_point(0., 0., -4.), Tuple::new_vector(0., 0., 1.));
    let xs = Intersections::new(vec![
        Intersection {
            object_ref: Box::new(Object::Sphere(A.clone())),
            time: 2.0,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(B.clone())),
            time: 2.75,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(C.clone())),
            time: 3.25,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(B)),
            time: 4.75,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(C)),
            time: 5.25,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(A)),
            time: 6.,
        },
    ]);
    println!("{:?}", xs.hits().unwrap().time);
    let mut i = 0;
    for x in xs.intersections.iter() {
        // println!("{:?}", x.clone());
        let comps = prepare_computations(x, r, &xs);
        let n1 = comps.n1;
        let n2 = comps.n2;
        println!("{i} | {n1} | {n2}");
        i += 1;
    }
}
#[test]
pub fn computing_under_point_below_the_surface() {
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
    let mut shape = glass_sphere();
    shape.set_transform(Matrix::identity_matrix(4).translation(0., 0., 1.));
    let i = Intersection {
        object_ref: Box::new(Object::Sphere(shape)),
        time: 5.0,
    };
    let xs = Intersections::new(vec![i.clone()]);
    let comps = prepare_computations(&i, r, &xs);
    assert!(comps.under_point.z > EPSILON / 2.);
    assert!(comps.point.z < comps.under_point.z);
}

#[test]
pub fn refracted_color_on_opaque_material() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
    let r = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
    let w = World {
        objects: vec![Object::Sphere(s1.clone()), Object::Sphere(s2.clone())],
        lights: vec![l1],
    };
    let xs = Intersections::new(vec![
        Intersection {
            object_ref: Box::new(Object::Sphere(s1.clone())),
            time: 2.0,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(s1.clone())),
            time: 6.,
        },
    ]);
    let comps = prepare_computations(&xs[0], r, &xs);
    let c = w.refracted_color(comps, 5);
    assert_eq!(c, Color::new(0., 0., 0.));
}

#[test]
pub fn refracted_under_internal_reflection() {
    let mut s1 = sphere::Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.transparency = 1.0;
    s1.material.refractive_index = 1.5;
    let mut s2 = sphere::Sphere::new();
    s2.set_transform(Matrix::identity_matrix(4).scale(0.5, 0.5, 0.5));
    let l1 = Light::new(Color::new(1., 1., 1.), Tuple::new_point(-10., 10., -10.));
    let r = Ray::new(
        Tuple::new_point(0., 0., 0.707),
        Tuple::new_vector(0., 1., 0.),
    );
    let w = World {
        objects: vec![Object::Sphere(s1.clone()), Object::Sphere(s2.clone())],
        lights: vec![l1],
    };
    let xs = Intersections::new(vec![
        Intersection {
            object_ref: Box::new(Object::Sphere(s1.clone())),
            time: -0.707,
        },
        Intersection {
            object_ref: Box::new(Object::Sphere(s1.clone())),
            time: 0.707,
        },
    ]);
    let comps = prepare_computations(&xs[1], r, &xs);
    let c = w.refracted_color(comps, 5);
    assert_eq!(c, Color::new(0., 0., 0.));
}
