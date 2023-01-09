use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::{average_truth_estimators, Truth};

pub struct WeightlessAverageProxiesMechanism;

impl VotingMechanism for WeightlessAverageProxiesMechanism {
    fn solve(
        &mut self,
        proxies: &[&impl TruthEstimator],
        _delegators: &[&impl TruthEstimator],
        _rankings: &[Rankings],
    ) -> Truth {
        average_truth_estimators!(proxies)
    }
}
