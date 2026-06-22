pub struct RiskProfile {
    pub probability: f64,
    pub impact: f64,
}

impl RiskProfile {
    pub fn assess(&self) -> bool {
        (self.probability * self.impact) > 0.7
    }
}
