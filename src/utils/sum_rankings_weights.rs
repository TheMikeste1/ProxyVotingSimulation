use crate::{Rankings, TruthEstimator, Weight};
use std::collections::HashMap;
use std::rc::Rc;

pub fn sum_rankings_weights(
    rankings: &[Rankings],
) -> Vec<(Rc<dyn TruthEstimator>, Weight)> {
    let mut summed_rankings_map = HashMap::new();

    // Add all ranking proxies to the list, summing their weights
    for ranking in rankings {
        for ranking in ranking.iter() {
            let id = ranking.proxy.get_id();
            let weight = ranking.weight;
            let entry = summed_rankings_map
                .entry(id)
                .or_insert((Rc::clone(&ranking.proxy), Weight::from(0f64)));
            entry.1 += weight;
        }
    }

    // Convert the HashMap to a Vec ordered by weight
    let mut summed_rankings: Vec<(Rc<dyn TruthEstimator>, Weight)> =
        summed_rankings_map
            .into_iter()
            .map(|(_, (p, w))| (p, w))
            .collect();

    summed_rankings.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap());

    summed_rankings
}
