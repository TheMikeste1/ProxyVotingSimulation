use crate::average_truth_estimators;
use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;

pub struct WeightlessAverageProxiesMechanism;

impl VotingMechanism for WeightlessAverageProxiesMechanism {
    fn solve(
        &mut self,
        proxies: &[&dyn TruthEstimator],
        _delegators: &[&dyn TruthEstimator],
        _rankings: &[Rankings],
    ) -> f64 {
        average_truth_estimators!(proxies)
    }
}
