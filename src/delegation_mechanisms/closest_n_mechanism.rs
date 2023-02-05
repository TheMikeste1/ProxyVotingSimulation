use crate::delegation_mechanisms::DelegationMechanism;
use crate::utils::sort_by_distance;
use crate::{Rankings, TruthEstimator, Weight};
use std::cmp::min;
use std::rc::Rc;

pub struct ClosestNMechanism {
    n: u32,
}

impl ClosestNMechanism {
    pub(crate) fn new(n: u32) -> ClosestNMechanism {
        if n == 0 {
            panic!("n must be greater than 0");
        }
        ClosestNMechanism { n }
    }
}

impl DelegationMechanism for ClosestNMechanism {
    fn delegate(
        &self,
        agent: &Rc<dyn TruthEstimator>,
        proxies: &[Rc<dyn TruthEstimator>],
    ) -> Rankings {
        // Assign the closest proxy all the weight, order the rest by distance
        let sorted_proxies = sort_by_distance(agent, proxies);

        let mut weights = vec![Weight::from(0.0); sorted_proxies.len()];
        let to_take = min(self.n, sorted_proxies.len() as u32);
        let weight: Weight = Weight::from(1.0 / to_take as f64);
        weights
            .iter_mut()
            .take(to_take as usize)
            .for_each(|w| *w = weight);

        let sorted_proxies = sorted_proxies
            .iter()
            .map(|(p, _)| Rc::clone(p))
            .collect::<Vec<_>>();
        Rankings::new_from_weights(sorted_proxies.as_slice(), &weights)
    }
}

impl Default for ClosestNMechanism {
    fn default() -> Self {
        Self::new(1)
    }
}
