use crate::voting_mechanisms::*;

pub struct MeanVotingMechanism;

impl VotingMechanism for MeanVotingMechanism {
    fn vote(&self, weighted_votes: &[WeightedVote]) -> f64 {
        let total_weight = weighted_votes.iter().map(|v| v.weight).sum::<f64>();

        weighted_votes
            .iter()
            .map(|v| v.vote * v.weight)
            .sum::<f64>()
            / total_weight
    }
}
