use crate::utils::calculate_distance;
use crate::TruthEstimator;
use ordered_float::OrderedFloat;
use std::ops::Deref;
use std::rc::Rc;

pub fn sort_by_distance(
    agent: &dyn TruthEstimator,
    proxies: &[Rc<dyn TruthEstimator>],
) -> Vec<(Rc<dyn TruthEstimator>, OrderedFloat<f64>)> {
    let mut sorted_proxies = proxies
        .iter()
        .map(|p| (Rc::clone(p), calculate_distance(agent, p.deref())))
        .collect::<Vec<_>>();
    sorted_proxies.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    sorted_proxies
}
