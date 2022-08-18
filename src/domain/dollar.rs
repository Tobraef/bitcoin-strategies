use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Dollar(f32);

impl Mul for Dollar {
    type Output = Dollar;

    fn mul(self, rhs: Self) -> Self::Output {
        Dollar(self.0 * rhs.0)
    }
}

impl Add for Dollar {
    type Output = Dollar;

    fn add(self, rhs: Self) -> Self::Output {
        Dollar(self.0 + rhs.0)
    }
}

impl Div for Dollar {
    type Output = Dollar;

    fn div(self, rhs: Self) -> Self::Output {
        Dollar(self.0 / rhs.0)
    }
}

impl From<f32> for Dollar {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl Into<f32> for Dollar {
    fn into(self) -> f32 {
        self.0
    }
}

impl AsRef<f32> for Dollar {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
