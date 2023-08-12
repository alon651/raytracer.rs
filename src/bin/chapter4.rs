use ray_tracer::{canvas, matrix, tuple};
fn main() {
    let size: f32 = 100.0;
    let iteration = 12;
    let mut c = canvas::Canvas::new(size as usize, size as usize);
    let mut p = tuple::Tuple::new_point(0.0, 0.0, size / 3.0);
    let t =
        matrix::Matrix::identity_matrix(4).rotate_y(std::f32::consts::PI / (iteration / 2) as f32);
    for _ in 0..iteration {
        p = &t * p;
        c.set_pixel(
            (p.x + size / 2.0) as usize,
            (p.z + size / 2.0) as usize,
            (255.0, 0.0, 0.0),
        );
    }

    c.save("canvas.ppm")
}
