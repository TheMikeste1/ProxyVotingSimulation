use crate::delegation_mechanisms::DelegationMechanism;
use crate::utils::sort_by_distance;
use crate::{Rankings, TruthEstimator};

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

impl<'a> DelegationMechanism<'a> for ClosestNMechanism {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[&'a dyn TruthEstimator],
    ) -> Rankings<'a> {
        // Assign the closest proxy all the weight, order the rest by distance
        let sorted_proxies = sort_by_distance(agent, proxies);

        let mut weights = vec![0.0; sorted_proxies.len()];
        let weight = 1.0 / self.n as f64;
        weights
            .iter_mut()
            .take(self.n as usize)
            .for_each(|w| *w = weight);

        Rankings::new_from_weights(
            &sorted_proxies.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
            &weights,
        )
    }
}

impl Default for ClosestNMechanism {
    fn default() -> Self {
        Self::new(1)
    }
}
