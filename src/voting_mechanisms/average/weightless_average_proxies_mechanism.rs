use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;

pub struct WeightlessAverageProxiesMechanism;

impl VotingMechanism for WeightlessAverageProxiesMechanism {
    fn solve(
        &mut self,
        _proxies: &[&dyn TruthEstimator],
        _delegators: &[&dyn TruthEstimator],
        _rankings: &[Rankings],
    ) -> f64 {
        todo!("Implement WeightlessAverageProxiesMechanism::solve")
    }
}
