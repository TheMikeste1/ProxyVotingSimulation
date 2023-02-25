use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;

pub struct ExpertCandidateCoordinationMechanism;

impl CoordinationMechanism for ExpertCandidateCoordinationMechanism {
    fn coordinate(&self, expert: &Agent, _delegations: &[&Agent]) -> f64 {
        expert.get_preference().round()
    }
}

impl ExpertCandidateCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ExpertCandidateCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
