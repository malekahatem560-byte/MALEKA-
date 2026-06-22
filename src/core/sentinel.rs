pub struct Sentinel {
    pub max_risk_threshold: f64,
}

impl Sentinel {
    pub fn new(threshold: f64) -> Self {
        Self { max_risk_threshold: threshold }
    }

    pub fn validate_transformation(&self, risk_score: f64) -> Result<(), String> {
        if risk_score > self.max_risk_threshold {
            return Err("SECURITY ALERT: Transformation exceeds safety threshold. Blocked.".to_string());
        }
        Ok(())
    }
}
