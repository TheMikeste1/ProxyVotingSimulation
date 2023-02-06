use crate::{Agent, Delegation};

pub trait MeanCoordinationMechanism {
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
