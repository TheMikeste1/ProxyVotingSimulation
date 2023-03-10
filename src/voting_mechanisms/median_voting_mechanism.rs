use itertools::Itertools;

use crate::voting_mechanisms::*;

pub struct MedianVotingMechanism;

impl VotingMechanism for MedianVotingMechanism {
    fn vote(&self, weighted_votes: &[WeightedVote]) -> f64 {
        let ordered_votes = weighted_votes
            .iter()
            .sorted_by(|a, b| a.vote.partial_cmp(&b.vote).unwrap())
            .collect::<Vec<_>>();
        let total_weight = weighted_votes.iter().map(|v| v.weight).sum::<f64>();
        let mut weight_sum = 0f64;
        for (i, WeightedVote { vote, weight }) in ordered_votes.iter().enumerate() {
            weight_sum += weight;
            if weight_sum == total_weight / 2f64 {
                return (*vote + ordered_votes[i + 1].vote) / 2f64;
            }
            if weight_sum > total_weight / 2f64 {
                return *vote;
            }
        }
        unreachable!("Sum of agents' weight did not reach total weight!");
    }
}

impl MedianVotingMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MedianVotingMechanism {
    fn default() -> Self {
        Self::new()
    }
}
