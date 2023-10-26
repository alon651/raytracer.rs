use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::matrix::Matrix;
use ray_tracer::object::Object;
use ray_tracer::patterns::Pattern;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

pub fn main(){
    let mut w = World::default();

    let mut m1 = Material::default();
    m1.reflective = 1.;
    m1.transparency = 0.9;
    m1.refractive_index = 1.5;
    m1.color = Color::new(0.,0.,0.);

    let mut s1 = Object::new_sphere();
    s1.material = m1.clone();
    s1.set_transform(Matrix::identity_matrix(4).translation(2.,1.,0.));
    w.push_obj(s1.clone());
    s1.set_transform(Matrix::identity_matrix(4).translation(-1.5,1.,0.));
    let mut pattern = Pattern::new_gradient_pattern(Color::new(1., 0., 0.), Color::new(0., 0., 1.));
    pattern.transform(Matrix::identity_matrix(4).scale(2.,2.,2.));
    s1.material.pattern = Some(pattern.clone());
    s1.material.transparency = 0.;
    // s1.material.refractive_index = 1.5;
    s1.material.reflective = 0.;
    w.push_obj(s1);
    let mut p = Object::new_plane();
    let mut pattern = Pattern::new_checkers_pattern(Color::new(0., 0., 0.), Color::new(1., 1., 1.));
    m1.pattern = Some(pattern);
    m1.reflective = 1.;
    p.material = m1;
    w.push_obj(p);

    let light = Light::new(Color::new(1.,1.,1.),Tuple::new_point(2.,2.,2.));
    w.lights.push(light);
    let mut camera = Camera::new(600, 600, 1.7);
    camera.set_transform(Matrix::view_transform(Tuple::new_point(0.5,1.5,3.),Tuple::new_point(0.,0.25,0.),Tuple::new_vector(0.,1.,0.)));
    let c = camera.render(w);
    c.save2png("chapter11");
}