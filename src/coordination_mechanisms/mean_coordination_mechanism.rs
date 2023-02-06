use crate::coordination_mechanisms::CoordinationMechanism;
use crate::{Agent, Delegation};

pub struct MeanCoordinationMechanism;

impl CoordinationMechanism for MeanCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[Delegation]) -> f64 {
        (delegatee.get_preference()
            + delegations
                .iter()
                .map(|d| d.get_constituent().get_preference())
                .sum::<f64>())
            / (
                delegations.len() + 1
                // +1 for the delegatee
            ) as f64
    }
}
