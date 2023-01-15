use crate::*;

pub struct Agent {
    extent: f64,
    id: u32,
    rng: StdRng,
    last_estimate: Option<Truth>,
}

impl Agent {
    pub fn new(id: u32, extent: f64, rng: StdRng) -> Self {
        Self {
            extent,
            id,
            rng,
            last_estimate: None,
        }
    }
}

impl HasID for Agent {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl TruthEstimator for Agent {
    fn get_last_estimate(&self) -> Option<Truth> {
        self.last_estimate
    }

    fn set_last_estimate(&mut self, estimate: Truth) {
        self.last_estimate = Some(estimate);
    }

    fn generate_new_estimate(
        &mut self,
        distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
        truth: &Truth,
    ) -> Truth {
        let min = -self.extent;
        let max = self.extent;
        let value = distribution.sample(&mut self.rng);
        let error = min + (max - min) * value;

        truth + error
    }
}
