use crate::prelude::{Rankings, TruthEstimator};
use crate::Truth;

pub trait VotingMechanism {
    fn solve(
        &mut self,
        proxies: &[&impl TruthEstimator],
        delegators: &[&impl TruthEstimator],
        rankings: &[Rankings],
    ) -> Truth;
}
