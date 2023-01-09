use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;

pub struct MedianMechanism;

impl VotingMechanism for MedianMechanism {
    fn solve(
        &mut self,
        _proxies: &[&impl TruthEstimator],
        _delegators: &[&impl TruthEstimator],
        _rankings: &[Rankings],
    ) -> Truth {
        todo!("Implement MedianMechanism::solve")
    }
}
