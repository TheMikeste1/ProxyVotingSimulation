use crate::Agent;

pub trait CoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent], proxy_weight: f64, constituent_weight: f64) -> f64;
}
