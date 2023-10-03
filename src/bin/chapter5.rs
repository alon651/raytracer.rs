use ray_tracer::{matrix::Matrix, tuple::Tuple, *};

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 10.0;
    let canvas_pixel = 500;
    let pixel_size = wall_size / (canvas_pixel as f32);
    let half: f32 = wall_size / 2.0;

    let mut canvas = canvas::Canvas::new(500, 500);
    let color = color::Color::new(1.0, 1.0, 0.0);
    let mut shape = sphere::Sphere::new();
    //shape.set_transform(Matrix::identity_matrix(4).scale(1.0, 2.0, 3.0));
    for y in 0..(canvas_pixel - 1) {
        let world_y = half - pixel_size * y as f32;
        for x in 0..(canvas_pixel - 1) {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::new_point(world_x, world_y, wall_z);
            let ray = ray::Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = ray.intersect(&shape);
            if let Some(i) = xs.hits() { canvas.set_pixel(x, y, color.get_rgb()) }
        }
    }

    canvas.save("canvas.ppm")
}
