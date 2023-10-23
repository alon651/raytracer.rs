use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::tuple::Tuple;

#[derive(Clone, PartialEq, Debug)]
pub struct Pattern {
    pub patterns: PatternType,
    pub transform: Matrix,
}

impl Pattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        match &self.patterns {
            PatternType::Stripe(p) => p.pattern_at(point),
            PatternType::Gradient(p) => p.pattern_at(point),
        }
    }
    pub fn new_stripe_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Stripe(StripePattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
        }
    }
    pub fn new_gradient_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Gradient(GradientPattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
        }
    }
    pub fn transform(&mut self,transformation:Matrix){
        self.transform = transformation
    }
}
#[derive(Clone,Debug,PartialEq)]
pub enum PatternType {
    Stripe(StripePattern),
    Gradient(GradientPattern)
}
#[derive(Clone,Debug,PartialEq)]
pub struct StripePattern {
    pub colors: [Color; 2],
}
impl StripePattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            return self.colors[0];
        }
        self.colors[1]
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct GradientPattern{
    pub colors: [Color; 2]
}
impl GradientPattern{
    pub fn pattern_at(&self,point:Tuple)->Color{
        let distance = self.colors[1] - self.colors[0];
        let fraction = point.x - point.x.floor();
        self.colors[0] + distance * fraction
    }
}
