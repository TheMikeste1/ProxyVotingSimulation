use crate::delegation_mechanisms::DelegationMechanism;
use crate::utils::calculate_distance;
use crate::{Ranking, Rankings, TruthEstimator};

pub struct ClosestNMechanism {
    n: u32,
}

impl ClosestNMechanism {
    fn new(n: u32) -> ClosestNMechanism {
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
        let mut sorted_proxies = proxies
            .iter()
            .map(|&p| (p, calculate_distance(agent, p)))
            .collect::<Vec<_>>();
        sorted_proxies.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

        let mut weights = vec![0.0; sorted_proxies.len()];
        let weight = 1.0 / self.n as f64;
        for i in 0..self.n as usize {
            weights[i] = weight;
        }

        let ranking_vec = sorted_proxies
            .into_iter()
            .map(|(p, _)| p)
            .enumerate()
            .zip(weights)
            .map(|((rank, p), w)| Ranking::new(p, rank as u32, w))
            .collect();

        Rankings::new(ranking_vec)
    }
}

impl Default for ClosestNMechanism {
    fn default() -> Self {
        Self::new(1)
    }
}
