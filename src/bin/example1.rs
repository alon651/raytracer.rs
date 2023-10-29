use std::f32::consts::PI;
use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::object::Object;
use ray_tracer::patterns::Pattern;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

pub fn main(){
    let mut w = World::new();
    let mut floor_pattern = Pattern::new_checkers_pattern(Color::new(0., 0., 0.), Color::new(1., 1., 1.));

    let mut floor = Object::new_plane();
    floor.material = Material::default();
    floor.material.reflective = 0.7;
    floor.material.pattern = Some(floor_pattern);
    w.push_obj(floor);

    let mut middle_pattern = Pattern::new_checkers_pattern(Color::new(1., 0., 0.), Color::new(0., 0., 1.));
    middle_pattern.resize(0.5, 0.5, 0.5);
    middle_pattern.rotate(0., 0., PI/9.);

    let mut middle = Object::new_sphere();
    middle.move_by(-0.5, 1., 0.5);
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.1;
    middle.material.pattern = Some(middle_pattern);
    w.push_obj(middle);

    let mut right = Object::new_sphere();
    right.move_by(1.5,0.5,-0.5);
    right.resize(0.5,0.5,0.5);


    right.material.color = Color::new(0., 0., 0.);
    right.material.diffuse = 0.;
    right.material.specular = 0.3;
    right.material.transparency = 1.;
    right.material.refractive_index = 1.5;

    w.push_obj(right);

    let mut left = Object::new_sphere();
    left.move_by(-1.5,0.33,-0.75);
    left.resize(0.333,0.333,0.333);
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    w.push_obj(left);

    w.lights.push(Light::new(
        Color::new(1., 1., 1.),
        Tuple::new_point(-10., 10., -10.),
    ));
    let mut camera = Camera::new(1000, 1000, PI / 3.);
    camera.set_view(
        Tuple::new_point(0., 1.5, -5.),
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(0., 1., 0.),
    );
    camera.render(w).save2png("example1");
}