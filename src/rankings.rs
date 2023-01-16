use crate::{Ranking, TruthEstimator, Weight};
use std::rc::Rc;

#[derive(Default)]
pub struct Rankings {
    rankings: Vec<Ranking>,
}

impl Rankings {
    pub fn new(rankings: &[Ranking]) -> Self {
        let rankings = rankings.to_vec();
        Self { rankings }
    }

    pub fn new_from_weights(
        proxies: &[Rc<dyn TruthEstimator>],
        weights: &[Weight],
    ) -> Self {
        let rankings = proxies
            .iter()
            .enumerate()
            .zip(weights)
            .map(|((rank, p), w)| Ranking::new(Rc::clone(p), rank as u32, *w))
            .collect();
        Self { rankings }
    }

    pub fn is_empty(&self) -> bool {
        self.rankings.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Ranking> {
        self.rankings.get(index)
    }

    pub fn len(&self) -> usize {
        self.rankings.len()
    }

    pub fn push(
        &mut self,
        proxy: Box<dyn TruthEstimator>,
        requested_ranking: u32,
        weight: Weight,
    ) {
        // Check if the ranking already exists
        if self
            .rankings
            .binary_search_by(|r| {
                r.requested_ranking.partial_cmp(&requested_ranking).unwrap()
            })
            .is_ok()
        {
            // If it does, panic
            panic!("Ranking already exists");
        };

        // Determine the position to place the ranking
        let pos = self
            .rankings
            .binary_search_by(|r| {
                r.requested_ranking.partial_cmp(&requested_ranking).unwrap()
            })
            .unwrap_or_else(|e| e);

        // Insert the ranking
        self.rankings
            .insert(pos, Ranking::new(proxy.into(), requested_ranking, weight));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Ranking> {
        self.rankings.iter()
    }
}
