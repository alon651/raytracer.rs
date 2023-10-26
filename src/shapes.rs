pub mod sphere;
pub mod plane;

use sphere::Sphere;
use plane::Plane;
#[derive(PartialEq, Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}