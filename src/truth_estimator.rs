use crate::{HasID, RngLockedDistribution};
use ordered_float::OrderedFloat;
use rand::rngs::StdRng;

pub type Truth = OrderedFloat<f64>;

pub trait TruthEstimator: HasID {
    fn get_last_estimate(&self) -> Option<Truth>;
    fn set_last_estimate(&mut self, estimate: Truth);

    fn estimate(
        &mut self,
        distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
    ) -> Truth {
        self.estimate_about(distribution, &Truth::from(0.0))
    }

    fn estimate_about(
        &mut self,
        distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
        truth: &Truth,
    ) -> Truth {
        let estimate = self.generate_new_estimate(distribution, truth);
        self.set_last_estimate(estimate);
        estimate
    }

    fn generate_new_estimate(
        &mut self,
        distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
        truth: &Truth,
    ) -> Truth;
}
