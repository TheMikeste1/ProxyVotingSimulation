use crate::{Agent, Delegation};

pub trait CoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[Delegation]) -> f64;
}
