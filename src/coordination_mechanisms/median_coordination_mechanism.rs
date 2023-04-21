use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;
use itertools::Itertools;

pub struct MedianCoordinationMechanism;

impl CoordinationMechanism for MedianCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent], proxy_weight: f64, constituent_weight: f64) -> f64 {
        let delegatee_preference = delegatee.get_current_preference();
        let total_constituent_weight = delegations.len() as f64 * constituent_weight;
        let total_weight = proxy_weight + total_constituent_weight;

        // Median
        let ordered_preferences = delegations
            .iter()
            .map(|d| (d.get_original_preference(), constituent_weight))
            .chain(std::iter::once((delegatee_preference, proxy_weight)))
            .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .collect_vec();

        let mut cumulative_weight = 0f64;
        for (i, (preference, weight)) in ordered_preferences.iter().enumerate() {
            cumulative_weight += weight;
            if cumulative_weight > total_weight / 2f64 {
                return *preference;
            }
            else if cumulative_weight == total_weight / 2f64 {
                return (*preference + ordered_preferences[i + 1].0) / 2f64;
            }
        }
        unreachable!("Did not properly sum weight")
    }
}

impl MedianCoordinationMechanism {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MedianCoordinationMechanism {
    fn default() -> Self {
        Self::new()
    }
}
