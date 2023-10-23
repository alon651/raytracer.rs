use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::intersections::Intersectable;
use ray_tracer::light::Light;
use ray_tracer::material::lighting;
use ray_tracer::object;
use ray_tracer::{material::Material, matrix::Matrix, ray::Ray, sphere, tuple::Tuple};
use rayon::prelude::*;
use std::sync::Mutex;

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 10.0;
    let canvas_pixel = 1024;
    let pixel_size = wall_size / (canvas_pixel as f32);
    let half: f32 = wall_size / 2.0;

    let canvas = Canvas::new(canvas_pixel, canvas_pixel);
    let mut shape = sphere::Sphere::new();

    shape.set_transform(Matrix::identity_matrix(4).scale(1.0, 1., 1.).rotate_x(1.5));
    //
    //shape material
    shape.material = Material::default();
    shape.material.color = Color::new(0., 1., 0.);

    //light source
    let light_position = Tuple::new_point(-10., 10., -10.);
    let light_color = Color::new(1., 1., 1.);
    let light = Light::new(light_color, light_position);

    // Create slices from the ranges
    let y_range: Vec<usize> = (0..canvas_pixel - 1).collect();
    let x_range: Vec<usize> = (0..canvas_pixel - 1).collect();

    // Wrap Canvas in a Mutex
    let canvas = Mutex::new(canvas);

    //convert shape from sphere object to object
    let mut shape = object::Object::Sphere(shape);

    // Parallel iteration using rayon's par_iter
    y_range.par_iter().for_each(|&y| {
        let world_y = half - pixel_size * y as f32;

        x_range.par_iter().for_each(|&x| {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            if let Some(hit) = ray.intersect(&shape).hits() {
                let point = ray.position(hit.time);
                let normal = shape.normal_at(point);
                let eye = -ray.direction;
                let color = lighting(shape.get_material(), &light, point, eye, normal, false);
                // Lock the Mutex to access the Canvas
                let mut canvas_lock = canvas.lock().unwrap();
                canvas_lock.set_pixel(x, y, color.get_rgb());
            }
        });
    });

    canvas.lock().unwrap().save2png("canvas");
}
