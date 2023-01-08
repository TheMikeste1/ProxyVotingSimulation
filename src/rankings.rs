use crate::{Ranking, TruthEstimator};

#[derive(Default)]
pub struct Rankings<'a> {
    rankings: Vec<Ranking<'a>>,
}

impl<'a> Rankings<'a> {
    pub fn new(rankings: &[Ranking<'a>]) -> Self {
        let rankings = rankings.to_vec();
        Self { rankings }
    }

    pub fn new_from_weights(
        proxies: &[&'a dyn TruthEstimator],
        weights: &[f64],
    ) -> Self {
        let rankings = proxies
            .iter()
            .enumerate()
            .zip(weights)
            .map(|((rank, &p), w)| Ranking::new(p, rank as u32, *w))
            .collect();
        Self { rankings }
    }

    pub fn is_empty(&self) -> bool {
        self.rankings.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Ranking<'_>> {
        self.rankings.get(index)
    }

    pub fn len(&self) -> usize {
        self.rankings.len()
    }

    pub fn push(
        &mut self,
        proxy: &'a dyn TruthEstimator,
        requested_ranking: u32,
        weight: f64,
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
            .insert(pos, Ranking::new(proxy, requested_ranking, weight));
    }
}
