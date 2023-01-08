use crate::average_truth_estimators;
use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;

pub struct WeightlessAverageAllMechanism;

impl VotingMechanism for WeightlessAverageAllMechanism {
    fn solve(
        &mut self,
        proxies: &[&dyn TruthEstimator],
        delegators: &[&dyn TruthEstimator],
        _rankings: &[Rankings],
    ) -> f64 {
        let proxy_avg = average_truth_estimators!(proxies);
        let delegator_avg = average_truth_estimators!(delegators);
        (proxy_avg + delegator_avg) / 2f64
    }
}
