use crate::utils::cmp_f32;
pub enum TupleType {
    Vector,
    Point,
    None,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    /// creates a new tuple that represnt a point.
    /// a point is a tuple with `w=1.0`.
    /// a point represnts a point in space
    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }
    /// creates a new tuple that represnt a vector.
    ///
    /// a vector is a tuple with `w=0.0`.
    /// a vector represents a direction and a distance.
    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }
    ///creates a new custom tuple
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }
    ///calculate the magnitude of a Tuple
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // pub fn get_type(&self) -> Result<TupleType, &str> {
    //     if cmp_f32(self.w, 1.0) {
    //         return Ok(TupleType::Point);
    //     }
    //     if cmp_f32(self.w, 0.0) {
    //         return Ok(TupleType::Vector);
    //     }
    //     Err("Tuple Doesnt have a valid w value")
    // }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        cmp_f32(self.x, other.x)
            && cmp_f32(self.y, other.y)
            && cmp_f32(self.z, other.z)
            && cmp_f32(self.w, other.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}
impl std::ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self::new(0.0, 0.0, 0.0, 0.0) - self
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl std::ops::Mul<Tuple> for Tuple {
    type Output = f32;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
