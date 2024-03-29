use std::f32::consts::PI;

use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::matrix::Matrix;
use ray_tracer::object::Object;
use ray_tracer::patterns::Pattern;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

fn main() {
    let mut world = World::default();
    //floor
    let mut ring = Pattern::new_checkers_pattern(Color::new(1., 0., 0.), Color::new(0., 0., 1.));
    ring.transform(
        Matrix::identity_matrix(4)
            .scale(0.5, 0.5, 0.5)
            .rotate_y(PI / 9.),
    );
    let mut floor = Object::new_plane();
    // floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let ring2 = Pattern::new_checkers_pattern(Color::new(1., 1., 1.), Color::new(0., 0., 0.));
    floor.material.reflective = 0.7;
    floor.material.pattern = Some(ring2);
    world.push_obj(floor);

    // middle sphere
    let mut middle = Object::new_sphere();
    middle.move_by(-0.5, 1., 0.5);
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.1;
    middle.material.pattern = Some(ring.clone());
    world.push_obj(middle);

    //right sphere
    let mut right = Object::new_sphere();
    right.move_by(1.5,0.5,-0.5);
    right.resize(0.5,0.5,0.5);


    right.material.color = Color::new(0., 0., 0.);
    right.material.diffuse = 0.;
    right.material.specular = 0.3;
    right.material.transparency = 1.;
    right.material.refractive_index = 1.5;

    world.push_obj(right);
    //left sphere
    let mut left = Object::new_sphere();
    left.move_by(-1.5,0.33,-0.75);
    left.resize(0.333,0.333,0.333);
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.push_obj(left);

    world.lights.push(Light::new(
        Color::new(1., 1., 1.),
        Tuple::new_point(-10., 10., -10.),
    ));
    let mut camera = Camera::new(1000, 1000, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        Tuple::new_point(0., 1.5, -5.),
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);
    canvas.save2png("chapter10");
}
