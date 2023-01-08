use crate::delegation_mechanisms::DelegationMechanism;
use crate::utils::calculate_distance;
use crate::{Ranking, Rankings, TruthEstimator};

#[derive(Default)]
pub struct ClosestMechanism;

impl ClosestMechanism {
    fn new() -> ClosestMechanism {
        ClosestMechanism
    }
}

impl<'a> DelegationMechanism<'a> for ClosestMechanism {
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
        weights[0] = 1.0;

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
