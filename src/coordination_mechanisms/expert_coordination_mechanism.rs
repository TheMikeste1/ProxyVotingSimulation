use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;

pub struct ExpertCoordinationMechanism;

impl CoordinationMechanism for ExpertCoordinationMechanism {
    fn coordinate(&self, expert: &Agent, _delegations: &[&Agent], _proxy_weight: f64, _constituent_weight: f64) -> f64 {
        expert.get_current_preference()
    }
}

impl ExpertCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ExpertCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
