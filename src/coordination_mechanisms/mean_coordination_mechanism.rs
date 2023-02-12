use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;

pub struct MeanCoordinationMechanism;

impl CoordinationMechanism for MeanCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent]) -> f64 {
        (delegatee.get_preference()
            + delegations.iter().map(|d| d.get_preference()).sum::<f64>())
            / (
                delegations.len() + 1
                // +1 for the delegatee
            ) as f64
    }
}

impl MeanCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MeanCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
