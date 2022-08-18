use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Bitcoin(f32);

impl Mul for Bitcoin {
    type Output = Bitcoin;

    fn mul(self, rhs: Self) -> Self::Output {
        Bitcoin(self.0 * rhs.0)
    }
}

impl Div for Bitcoin {
    type Output = Bitcoin;

    fn div(self, rhs: Self) -> Self::Output {
        Bitcoin(self.0 / rhs.0)
    }
}

impl Add for Bitcoin {
    type Output = Bitcoin;

    fn add(self, rhs: Self) -> Self::Output {
        Bitcoin(self.0 + rhs.0)
    }
}

impl From<f32> for Bitcoin {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl Into<f32> for Bitcoin {
    fn into(self) -> f32 {
        self.0
    }
}

impl AsRef<f32> for Bitcoin {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
