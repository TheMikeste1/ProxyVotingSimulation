use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;

pub struct MeanCandidateCoordinationMechanism;

impl CoordinationMechanism for MeanCandidateCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent]) -> f64 {
        ((delegatee.get_preference()
            + delegations.iter().map(|d| d.get_preference()).sum::<f64>())
            / (
                delegations.len() + 1
                // +1 for the delegatee
            ) as f64)
            .round()
    }
}

impl MeanCandidateCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MeanCandidateCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
