use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;

pub struct RankedChoiceMechanism;

impl VotingMechanism for RankedChoiceMechanism {
    fn solve(
        &mut self,
        _proxies: &[&dyn TruthEstimator],
        _delegators: &[&dyn TruthEstimator],
        _rankings: &[Rankings],
    ) -> f64 {
        todo!("Implement RankedChoiceMechanism::solve")
    }
}
