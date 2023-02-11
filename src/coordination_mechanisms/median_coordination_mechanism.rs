use crate::coordination_mechanisms::CoordinationMechanism;
use crate::{Agent, Delegation};
use itertools::Itertools;

pub struct MedianCoordinationMechanism;

impl CoordinationMechanism for MedianCoordinationMechanism {
    fn coordinate(&self, delegatee: &Agent, delegations: &[Delegation]) -> f64 {
        let preferences = delegations
            .iter()
            .map(|d| d.get_constituent().get_preference())
            .merge([delegatee.get_preference()])
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect::<Vec<f64>>();
        let len = preferences.len();
        preferences[len / 2]
    }
}
