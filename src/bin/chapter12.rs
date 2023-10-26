use std::f32::consts::PI;
use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::matrix::Matrix;
use ray_tracer::object::Object;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

pub fn main(){
    let mut w = World::default();
    let mut s1 = Object::new_cube();
    s1.material.color = Color::new(1.,0.,0.);
    s1.material.transparency = 0.6;

    s1.set_transform(Matrix::identity_matrix(4).rotate_y(std::f32::consts::FRAC_PI_4));
    w.push_obj(s1);

    let light = Light::new(Color::new(1.,1.,1.),Tuple::new_point(-10., 10., -10.));
    w.lights.push(light);

    let mut sphere = Object::new_sphere();
    sphere.set_transform(Matrix::identity_matrix(4).translation(0.,0.,2.));
    w.push_obj(sphere);
    let mut camera = Camera::new(600, 600, PI/3.);
    camera.set_transform(Matrix::view_transform(
        Tuple::new_point(0., 1., -5.),
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(0., 1., 0.),
    ));    let c = camera.render(w);
    c.save2png("chapter12");
}