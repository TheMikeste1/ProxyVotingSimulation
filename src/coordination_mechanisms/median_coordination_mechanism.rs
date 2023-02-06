use crate::coordination_mechanisms::CoordinationMechanism;
use crate::{Agent, Delegation};

pub struct MedianCoordinationMechanism;

impl CoordinationMechanism for MedianCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[Delegation]) -> f64 {
        let mut preferences = delegations
            .iter()
            .map(|d| d.get_constituent().get_preference())
            .collect::<Vec<f64>>();
        preferences.push(delegatee.get_preference());
        preferences.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = preferences.len();
        if len % 2 == 0 {
            (preferences[len / 2 - 1] + preferences[len / 2]) / 2.0
        } else {
            preferences[len / 2]
        }
    }
}
