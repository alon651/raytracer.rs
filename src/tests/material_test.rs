use crate::color::Color;
use crate::light::Light;
use crate::material::{lighting, Material};
use crate::object::Object;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[test]
fn eye_between_light_and_surface() {
    let m = Material::default();
    let position = Tuple::new_point(0.0, 0.0, 0.0);

    let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new(Color::new(1., 1., 1.), Tuple::new_point(0., 0., -10.));
    let result = lighting(
        &m,
        &Object::Sphere(Sphere::new()),
        &light,
        position,
        eyev,
        normalv,
        false,
    );
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn eye_between_light_and_surface_offset45() {
    let m = Material::default();
    let position = Tuple::new_point(0.0, 0.0, 0.0);

    let eyev = Tuple::new_vector(0., 0.707, -0.707);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new(Color::new(1., 1., 1.), Tuple::new_point(0., 0., -10.));
    let result = lighting(
        &m,
        &Object::Sphere(Sphere::new()),
        &light,
        position,
        eyev,
        normalv,
        false,
    );
    assert_eq!(result, Color::new(1., 1., 1.));
}

#[test]
fn lighting_with_surface_in_the_shadow() {
    let m = Material::default();
    let position = Tuple::new_point(0.0, 0.0, 0.0);

    let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new(Color::new(1., 1., 1.), Tuple::new_point(0., 0., -10.));
    let result = lighting(
        &m,
        &Object::Sphere(Sphere::new()),
        &light,
        position,
        eyev,
        normalv,
        true,
    );
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
