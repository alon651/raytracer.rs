use std::f32::consts::PI;

use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::matrix::Matrix;
use ray_tracer::object::Object;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

fn main() {
    let mut world = World::default();
    //floor
    let mut floor = Object::new_plane();
    floor.set_transform(Matrix::identity_matrix(4));
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;
    world.push_obj(floor);

    // middle sphere
    let mut middle = Object::new_sphere();
    middle.set_transform(Matrix::identity_matrix(4).translation(-0.5, 1., 0.5));
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.push_obj(middle);

    //right sphere
    let mut right = Object::new_sphere();
    right.set_transform(
        Matrix::identity_matrix(4)
            .translation(1.5, 0.5, -0.5)
            .scale(0.5, 0.5, 0.5),
    );
    right.material.color = Color::new(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.push_obj(right);

    //left sphere
    let mut left = Object::new_sphere();
    left.set_transform(
        Matrix::identity_matrix(4)
            .translation(-1.5, 0.33, -0.75)
            .scale(0.33, 0.33, 0.33),
    );
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.push_obj(left );

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
    canvas.save2png("chapter9");
}
