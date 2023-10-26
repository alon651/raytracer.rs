use crate::color::Color;
use crate::light::Light;
use crate::object::Object;
use crate::patterns::Pattern;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Color,
    pub pattern: Option<Pattern>,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        color: Color,
        reflective: f32,
        transparency: f32,
        refractive_index: f32,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
            color,
            pattern: None,
            reflective,
            transparency,
            refractive_index,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(0.1, 0.9, 0.9, 200.0, Color::new(1., 1., 1.), 0., 0., 1.)
    }
}

pub fn lighting(
    material: &Material,
    object: &Object,
    light: &Light,
    point: Tuple,
    eyev: Tuple,
    normnalv: Tuple,
    in_shadow: bool,
) -> Color {
    let color = if let Some(pattern) = &material.pattern {
        object.stripe_at_object(pattern, point)
    } else {
        material.color
    };
    let effective_color = color * light.intensity;
    let lightv = (light.position - point).normalize();
    let diffuse: Color;
    let specular: Color;
    let ambient = effective_color * material.ambient;

    let light_dot_normal = lightv * normnalv;
    if light_dot_normal < 0. {
        diffuse = Color::new(0., 0., 0.);
        specular = Color::new(0., 0., 0.);
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflect_v = (-lightv).reflect(normnalv);
        let reflect_dot_eye = reflect_v * eyev;
        if reflect_dot_eye <= 0. {
            specular = Color::new(0., 0., 0.);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    };
    if in_shadow {
        return ambient;
    };
    ambient + diffuse + specular
}
