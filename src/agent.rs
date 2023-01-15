use crate::*;
use rand::distributions::Distribution;
use rand::Rng;

pub struct Agent<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    extent: f64,
    id: u32,
    preference_distribution: PreferenceDistribution<R, D>,
    last_estimate: Option<Truth>,
}

impl<R, D> Agent<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    pub fn new(
        id: u32,
        extent: f64,
        preference_distribution: PreferenceDistribution<R, D>,
    ) -> Self {
        Self {
            extent,
            id,
            preference_distribution,
            last_estimate: None,
        }
    }
}

impl<R, D> HasID for Agent<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl<R, D> TruthEstimator for Agent<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    fn get_last_estimate(&self) -> Option<Truth> {
        self.last_estimate
    }

    fn estimate(&mut self, truth: &Truth) -> Truth {
        let error = self
            .preference_distribution
            .generate_value(-self.extent, self.extent);
        let estimate = truth + error;
        self.last_estimate = Some(estimate);
        estimate
    }
}
