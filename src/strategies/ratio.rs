use std::ops::Div;

pub struct Ratio<F> {
    previous_value: F,
    current_value: F,
}

impl<F> Ratio<F>
where F: Into<f32> + Div<Output = F> + Copy {
    pub fn new(previous_value: F, current_value: F) -> Self { Self { previous_value, current_value } }

    pub fn rule_applies(&self, boundary: i32) -> bool {
        assert!(boundary > 0);
        let f_ratio = boundary as f32 / 100.;
        let actual_ratio: f32 = (self.previous_value / self.current_value).into();
        actual_ratio < f_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::Ratio;

    #[test]
    fn should_apply_rule_correctly() {
        let sut = Ratio::new(5., 10.);

        assert!(!sut.rule_applies(25));
        assert!(sut.rule_applies(75));
    }
}
