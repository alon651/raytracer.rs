use crate::color::Color;
use crate::light::Light;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Color,
}

impl Material {
    pub fn new(
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        color: Color,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
            color,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(0.1, 0.9, 0.9, 200.0, Color::new(1., 1., 1.))
    }
}

pub fn lighting(
    material: &Material,
    light: &Light,
    point: Tuple,
    eyev: Tuple,
    normnalv: Tuple,
    in_shadow: bool,
) -> Color {
    let effective_color = material.color * light.intensity;
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
