use ray_tracer::{canvas, tuple::Tuple};

pub struct Projectile {
    pub position_point: Tuple,
    pub velocity_vector: Tuple,
}
pub struct Environment {
    pub gravity_vector: Tuple,
    pub wind_vector: Tuple,
}
fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position_point + projectile.velocity_vector;
    let velocity =
        projectile.velocity_vector + environment.gravity_vector + environment.wind_vector;
    Projectile {
        position_point: position,
        velocity_vector: velocity,
    }
}
fn main() {
    let mut c = canvas::Canvas::new(900, 550);
    let mut p = Projectile {
        position_point: Tuple::new_point(0.0, 1.0, 0.0),
        velocity_vector: Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity_vector: Tuple::new_vector(0.0, -0.1, 0.0),
        wind_vector: Tuple::new_vector(-0.01, 0.0, 0.0),
    };
    let mut ticks = 0;
    while p.position_point.y > 0.0 {
        p = tick(&e, &p);
        println!("{:?}", p.position_point);
        ticks += 1;
        c.set_pixel(
            p.position_point.x.round() as usize,
            c.height - p.position_point.y.round() as usize,
            (100.0, 100.0, 100.0),
        );
    }
    println!("{ticks} ticks");
    c.save("canvas.ppm");
}
