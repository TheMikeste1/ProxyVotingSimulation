use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;

pub struct MeanCoordinationMechanism;

impl CoordinationMechanism for MeanCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent], proxy_weight: f64, constituent_weight: f64) -> f64 {
        let delegatee_preference = delegatee.get_current_preference();
        let total_constituent_weight = delegations.len() as f64 * constituent_weight;

        (delegatee_preference * proxy_weight
            + delegations
                .iter()
                .map(
                    |d| d.get_original_preference() * constituent_weight, // Delegators can't participate, so they use their original preference
                )
                .sum::<f64>())
            / (
                proxy_weight + total_constituent_weight
            )
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
