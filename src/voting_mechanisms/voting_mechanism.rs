pub struct WeightedVote {
    pub vote: f64,
    pub weight: f64,
}

pub trait VotingMechanism {
    fn vote(&self, weighted_votes: &[WeightedVote]) -> f64;
}
