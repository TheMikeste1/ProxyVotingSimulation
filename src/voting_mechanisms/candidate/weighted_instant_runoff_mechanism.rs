use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;

pub struct WeightedInstantRunoffMechanism;

impl VotingMechanism for WeightedInstantRunoffMechanism {
    fn solve(
        &mut self,
        _proxies: &[&impl TruthEstimator],
        _delegators: &[&impl TruthEstimator],
        _rankings: &[Rankings],
    ) -> Truth {
        todo!("Implement WeightedInstantRunoffMechanism::solve")
    }
}
