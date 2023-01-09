use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::{average_truth_estimators, Truth};

pub struct WeightlessAverageAllMechanism;

impl VotingMechanism for WeightlessAverageAllMechanism {
    fn solve(
        &mut self,
        proxies: &[&impl TruthEstimator],
        delegators: &[&impl TruthEstimator],
        _rankings: &[Rankings],
    ) -> Truth {
        let proxy_avg = average_truth_estimators!(proxies);
        let delegator_avg = average_truth_estimators!(delegators);
        (proxy_avg + delegator_avg) / 2f64
    }
}
