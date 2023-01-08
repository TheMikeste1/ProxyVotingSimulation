use crate::prelude::{Rankings, TruthEstimator};

pub trait VotingMechanism {
    fn solve(
        &mut self,
        proxies: &[&dyn TruthEstimator],
        delegators: &[&dyn TruthEstimator],
        rankings: &[Rankings],
    ) -> f64;
}
