use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;
use itertools::Itertools;

pub struct MedianCandidateCoordinationMechanism;

impl CoordinationMechanism for MedianCandidateCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent]) -> f64 {
        let preferences = delegations
            .iter()
            .map(|d| d.get_preference())
            .merge([delegatee.get_preference()])
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect::<Vec<f64>>();
        let len = preferences.len();
        preferences[len / 2].round()
    }
}

impl MedianCandidateCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MedianCandidateCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
