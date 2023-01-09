use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;

pub struct InstantRunoffMechanism;

impl VotingMechanism for InstantRunoffMechanism {
    fn solve(
        &mut self,
        _proxies: &[&impl TruthEstimator],
        _delegators: &[&impl TruthEstimator],
        _rankings: &[Rankings],
    ) -> Truth {
        todo!("Implement InstantRunoffMechanism::solve")
    }
}
