use crate::utils::calculate_distance;
use crate::TruthEstimator;

pub fn sort_by_distance<'a>(
    agent: &dyn TruthEstimator,
    proxies: &[&'a dyn TruthEstimator],
) -> Vec<(&'a dyn TruthEstimator, f64)> {
    let mut sorted_proxies = proxies
        .iter()
        .map(|&p| (p, calculate_distance(agent, p)))
        .collect::<Vec<_>>();
    sorted_proxies.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    sorted_proxies
}
