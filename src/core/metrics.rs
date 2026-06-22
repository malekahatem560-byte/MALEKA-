pub struct Metrics {
    pub cci: f64,
    pub stability: f64,
}

impl Metrics {
    pub fn calculate_health(&self) -> f64 {
        self.cci * self.stability
    }
}
