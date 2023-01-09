use crate::*;
use rand::prelude::Distribution;

pub struct Agent<'pd, PD>
where
    PD: Distribution<f64>,
{
    extent: f64,
    id: u32,
    preference_distribution: PreferenceDistribution<'pd, PD>,
    last_estimate: Option<Truth>,
}

impl<'pd, PD> Agent<'pd, PD>
where
    PD: Distribution<f64>,
{
    pub fn new(
        id: u32,
        extent: f64,
        preference_distribution: PreferenceDistribution<'pd, PD>,
    ) -> Self {
        Self {
            extent,
            id,
            preference_distribution,
            last_estimate: None,
        }
    }
}

impl<PD> HasID for Agent<'_, PD>
where
    PD: Distribution<f64>,
{
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl<PD> TruthEstimator for Agent<'_, PD>
where
    PD: Distribution<f64>,
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
