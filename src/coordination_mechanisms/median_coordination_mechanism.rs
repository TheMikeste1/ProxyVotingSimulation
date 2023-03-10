use crate::coordination_mechanisms::CoordinationMechanism;
use crate::Agent;
use itertools::Itertools;

pub struct MedianCoordinationMechanism;

impl CoordinationMechanism for MedianCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[&Agent]) -> f64 {
        let preferences = delegations
            .iter()
            .map(|d| d.get_preference())
            .merge([delegatee.get_preference()])
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect::<Vec<f64>>();
        let len = preferences.len();
        if len % 2 == 0 {
            (preferences[len / 2 - 1] + preferences[len / 2]) / 2.0
        } else {
            preferences[len / 2]
        }
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
