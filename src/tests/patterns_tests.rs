use crate::color::Color;
use crate::intersections::Intersectable;
use crate::light::Light;
use crate::material::{lighting, Material};
use crate::matrix::Matrix;
use crate::object::Object::Sphere;
use crate::patterns::Pattern;
use crate::sphere;
use crate::tuple::Tuple;

static BLACK: Color = Color {
    color_tuple: Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.,
    },
};
static WHITE: Color = Color {
    color_tuple: Tuple {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.,
    },
};
#[test]
pub fn create_stripe_pattern() {
    let pattern = Pattern::new_stripe_pattern(WHITE, BLACK);
    assert_eq!(
        pattern,
        Pattern::new_stripe_pattern(WHITE,BLACK)
        // Pattern::new_stripe{
        //     a: WHITE,
        //     b: BLACK,
        //     transform: Matrix::identity_matrix(4)
        // }
    )
}

#[test]
pub fn y_stripe_pattern() {
    let pattern = Pattern::new_stripe_pattern(WHITE, BLACK);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 0., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 1., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 2., 0.,)), WHITE);
}

#[test]
pub fn z_stripe_pattern() {
    let pattern = Pattern::new_stripe_pattern(WHITE, BLACK);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 0., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 0., 1.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 0., 2.,)), WHITE);
}

#[test]
pub fn x_stripe_pattern() {
    let pattern = Pattern::new_stripe_pattern(WHITE, BLACK);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0., 0., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(1., 0., 0.,)), BLACK);
    assert_eq!(pattern.pattern_at(Tuple::new_point(2., 0., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(0.9, 0., 0.,)), WHITE);
    assert_eq!(pattern.pattern_at(Tuple::new_point(-1., 0., 0.,)), BLACK);
    assert_eq!(pattern.pattern_at(Tuple::new_point(1.1, 0., 0.,)), BLACK);
}

#[test]
pub fn lightning_with_a_pattern() {
    let mut m = Material::default();
    m.pattern = Some(Pattern::new_stripe_pattern(WHITE, BLACK));
    m.ambient = 1.;
    m.diffuse = 0.;
    m.specular = 0.;
    let eyev = Tuple::new_vector(0., 0., -1.);
    let normalv = Tuple::new_vector(0., 0., -1.);
    let light = Light::new(Color::new(1., 1., 1.), Tuple::new_point(0., 0., -10.));
    let c1 = lighting(
        &m,
        &Sphere(sphere::Sphere::new()),
        &light,
        Tuple::new_point(0.9, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    let c2 = lighting(
        &m,
        &Sphere(sphere::Sphere::new()),
        &light,
        Tuple::new_point(1.1, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    assert_eq!(c1, WHITE);
    assert_eq!(c2, BLACK);
}

#[test]
pub fn stripes_with_object_transformation() {
    let mut obj = sphere::Sphere::new();
    obj.set_transform(Matrix::identity_matrix(4).scale(2., 2., 2.));
    let pattern = Pattern::new_stripe_pattern(WHITE, BLACK);
    let obj = Sphere(obj);
    let c = obj.stripe_at_object(&pattern, Tuple::new_point(1.5, 0., 0.));
    assert_eq!(c, WHITE);
}
