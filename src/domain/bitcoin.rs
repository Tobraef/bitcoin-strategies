use derive_more::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Add, Mul, Div, Sub, From, Into)]
pub struct Bitcoin(f32);

impl AsRef<f32> for Bitcoin {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
