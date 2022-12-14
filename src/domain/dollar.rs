use derive_more::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq, PartialOrd, Add, Mul, Div, Sub, From, Into)]
pub struct Dollar(f32);

impl AsRef<f32> for Dollar {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
