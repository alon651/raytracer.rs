use std::f32::consts::PI;

use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::matrix::Matrix;
use ray_tracer::object::Object;
use ray_tracer::patterns::Pattern;
use ray_tracer::plane::Plane;
use ray_tracer::sphere::Sphere;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

fn main() {
    let mut world = World::default();
    let mut p = Pattern::new_stripe_pattern(Color::new(1., 0., 0.), Color::new(0., 0., 1.));
    p.transform(
        Matrix::identity_matrix(4)
            .scale(0.5, 0.5, 0.5)
            .rotate_y(0.9),
    );
    //floor
    let mut floor = Plane::new();
    floor.transform = Matrix::identity_matrix(4);
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;
    floor.material.pattern = Some(p.clone());
    world.push_obj(Object::Plane(floor));
    let mut ring = Pattern::new_checkers_pattern(Color::new(1., 0., 0.), Color::new(0., 0., 1.));
    ring.transform(Matrix::identity_matrix(4).scale(0.05,0.05,0.05).rotate_y(PI/9.));
    // middle sphere
    let mut middle = Sphere::new();
    middle.transform = Matrix::identity_matrix(4).translation(-0.5, 1., 0.5);
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.pattern = Some(ring);
    world.push_obj(Object::Sphere(middle));

    //right sphere
    let mut right = Sphere::new();
    right.transform = Matrix::identity_matrix(4)
        .translation(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.push_obj(Object::Sphere(right));
    //left sphere
    p.transform = Matrix::identity_matrix(4);
    let mut left = Sphere::new();
    left.transform = Matrix::identity_matrix(4)
        .translation(-1.5, 0.33, -0.75)
        .scale(0.33, 0.33, 0.33);
    left.material.color = Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.push_obj(Object::Sphere(left));

    world.lights.push(Light::new(
        Color::new(1., 1., 1.),
        Tuple::new_point(-10., 10., -10.),
    ));
    let mut camera = Camera::new(1000, 1000, PI / 3.);
    camera.transform = Matrix::view_transform(
        Tuple::new_point(0., 1.5, -5.),
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(0., 1., 0.),
    );

    let canvas = camera.render(world);
    canvas.save2png("chapter10");
}
