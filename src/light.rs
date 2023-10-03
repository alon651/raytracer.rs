use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct Light {
    pub intensity:Color,
    pub position:Tuple
}

impl Light {
    pub fn new(intensity:Color,position:Tuple)->Light{
        Light{intensity,position}
    }
}   