use crate::camera::Camera;
use crate::color::Color;
use crate::intersections::Intersectable;
use crate::light::Light;
use crate::matrix::Matrix;
use crate::object::Object::Sphere;
use crate::sphere;
use crate::tuple::Tuple;
use crate::world::World;
use std::f32::consts::PI;

#[test]
fn creating_a_camera() {
    let hsize = 100;
    let vsize = 120;
    let field_of_view = PI / 2.;
    let c = Camera::new(hsize, vsize, field_of_view);
    assert_eq!(c.transform, Matrix::identity_matrix(4))
}

#[test]
fn pixel_size_on_horizontal_aspect() {
    let c = Camera::new(200, 125, PI / 2.);
    assert_eq!(c.pixel_size, 0.01)
}

#[test]
fn pixel_size_on_vertical_aspect() {
    let c = Camera::new(125, 200, PI / 2.);
    assert_eq!(c.pixel_size, 0.01)
}

#[test]
fn ray_trough_center() {
    let c = Camera::new(201, 101, PI / 2.);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, Tuple::new_point(0., 0., 0.));
    assert_eq!(r.direction, Tuple::new_vector(0., 0., -1.));
}
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
fn render_test() {
    let w = default_world();
    let mut c = Camera::new(11, 11, PI / 2.);
    let from = Tuple::new_point(0., 0., -5.);
    let to = Tuple::new_point(0., 0., 0.);
    let up = Tuple::new_vector(0., 1., 0.);
    c.transform = Matrix::view_transform(from, to, up);
    let im = c.render(w);
    let p = im.get_pixel(5, 5);
    println!("{p:?}");
    assert_eq!(
        p,
        Color {
            color_tuple: Tuple {
                x: 0.38066125,
                y: 0.4758265,
                z: 0.28549594,
                w: 1.0
            }
        }
    )
}
