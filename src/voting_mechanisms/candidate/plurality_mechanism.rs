use crate::prelude::{Rankings, TruthEstimator};
use crate::utils::sum_rankings_weights;
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;

pub struct PluralityMechanism;

impl VotingMechanism for PluralityMechanism {
    fn solve(
        &mut self,
        _proxies: &[&impl TruthEstimator],
        _delegators: &[&impl TruthEstimator],
        rankings: &[Rankings],
    ) -> Truth {
        let summed_rankings = sum_rankings_weights(rankings);
        summed_rankings
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0
            .get_last_estimate()
            .unwrap()
    }
}
