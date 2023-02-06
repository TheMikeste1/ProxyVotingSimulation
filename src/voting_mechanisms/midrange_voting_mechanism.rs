use itertools::Itertools;

use crate::voting_mechanisms::*;

pub struct MidrangeVotingMechanism;

impl VotingMechanism for MidrangeVotingMechanism {
    fn vote(&self, weighted_votes: &[WeightedVote]) -> f64 {
        let ordered_votes = weighted_votes
            .iter()
            .sorted_by(|a, b| a.vote.partial_cmp(&b.vote).unwrap())
            .collect::<Vec<_>>();

        // TODO: Should this be weighted? See my note in the MidRange equation in the paper.
        (ordered_votes.first().unwrap().vote + ordered_votes.last().unwrap().vote) / 2.0
    }
}
