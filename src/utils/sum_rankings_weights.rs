use crate::{Rankings, TruthEstimator, Weight};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct ProxyWeightSum {
    pub proxy: Weak<dyn TruthEstimator>,
    pub weight: Weight,
}

pub fn sum_rankings_weights(rankings: &[Rankings]) -> Vec<ProxyWeightSum> {
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
    let mut summed_rankings: Vec<ProxyWeightSum> = summed_rankings_map
        .into_iter()
        .map(|(_, (p, w))| ProxyWeightSum {
            proxy: Rc::downgrade(&p),
            weight: w,
        })
        .collect();

    summed_rankings.sort_by(|a, b| a.weight.partial_cmp(b.weight.borrow()).unwrap());

    summed_rankings
}
