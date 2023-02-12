use crate::Agent;

pub trait CoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent]) -> f64;
}
