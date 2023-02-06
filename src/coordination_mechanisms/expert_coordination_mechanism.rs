use crate::coordination_mechanisms::CoordinationMechanism;
use crate::{Agent, Delegation};

pub struct ExpertCoordinationMechanism;

impl CoordinationMechanism for ExpertCoordinationMechanism {
    fn coordinate(&self, expert: &Agent, _delegations: &[Delegation]) -> f64 {
        expert.get_preference()
    }
}
