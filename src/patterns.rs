use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Clone, PartialEq, Debug)]
pub struct Pattern {
    pub patterns: PatternType,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
}

impl Pattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        match &self.patterns {
            PatternType::Stripe(p) => p.pattern_at(point),
            PatternType::Gradient(p) => p.pattern_at(point),
            PatternType::Ring(p) => p.pattern_at(point),
            PatternType::Checkers(p) => p.pattern_at(point),
        }
    }
    pub fn new_stripe_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Stripe(StripePattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
            inverse_transform: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn new_gradient_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Gradient(GradientPattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
            inverse_transform: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn new_ring_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Ring(RingPattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
            inverse_transform: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn new_checkers_pattern(a: Color, b: Color) -> Pattern {
        Pattern {
            patterns: PatternType::Checkers(CheckersPattern { colors: [a, b] }),
            transform: Matrix::identity_matrix(4),
            inverse_transform: Matrix::identity_matrix(4).inverse(),
        }
    }
    pub fn transform(&mut self, transformation: Matrix) {
        self.transform = transformation;
        self.inverse_transform = self.transform.inverse();
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum PatternType {
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkers(CheckersPattern),
}
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
    pub colors: [Color; 2],
}
impl GradientPattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.colors[1] - self.colors[0];
        let fraction = point.x - point.x.floor();
        self.colors[0] + distance * fraction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    pub colors: [Color; 2],
}
impl RingPattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        let dist = (point.x * point.x + point.z * point.z).sqrt();
        if dist.floor() as i16 % 2 == 0 {
            return self.colors[0];
        }
        self.colors[1]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CheckersPattern {
    pub colors: [Color; 2],
}
impl CheckersPattern {
    pub fn pattern_at(&self, point: Tuple) -> Color {
        let a = point.x.floor() + point.y.floor() + point.z.floor();
        if a as i16 % 2 == 0 {
            return self.colors[0];
        }
        self.colors[1]
    }
}
