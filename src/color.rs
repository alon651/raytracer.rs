use crate::tuple::Tuple;
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    color_tuple: Tuple,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            color_tuple: Tuple::new(r, g, b, 1.0),
        }
    }
    pub fn get_rgb(&self) -> (f32, f32, f32) {
        (self.color_tuple.x, self.color_tuple.y, self.color_tuple.z)
    }
}
impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        (self.color_tuple + rhs.color_tuple).into()
    }
}
impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.color_tuple - rhs.color_tuple).into()
    }
}
impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.color_tuple * rhs).into()
    }
}
impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        (Tuple::new(
            self.color_tuple.x * rhs.color_tuple.x,
            self.color_tuple.y * rhs.color_tuple.y,
            self.color_tuple.z * rhs.color_tuple.z,
            1.0,
        ))
        .into()
    }
}
impl From<Tuple> for Color {
    fn from(value: Tuple) -> Self {
        Color {
            color_tuple: Tuple::new_point(value.x, value.y, value.z),
        }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Tuple::new(value.0, value.1, value.2, 1.0).into()
    }
}
