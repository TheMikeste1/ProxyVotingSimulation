use crate::{Ranking, TruthEstimator};

#[derive(Default)]
pub struct Rankings<'a> {
    rankings: Vec<Ranking<'a>>,
}

impl<'a> Rankings<'a> {
    pub fn new(rankings: Vec<Ranking<'a>>) -> Self {
        Self { rankings }
    }

    pub fn add(&mut self, proxy: &'a dyn TruthEstimator, requested_ranking: u32, weight: f64) {
        // let ranking = Ranking::new(proxy, ranking, weight);
        // Check if the ranking already exists
        if self
            .rankings
            .binary_search_by(|r| r.requested_ranking.partial_cmp(&requested_ranking).unwrap())
            .is_ok()
        {
            // If it does, panic
            panic!("Ranking already exists");
        };

        // Determine the position to place the ranking
        let pos = self
            .rankings
            .binary_search_by(|r| r.requested_ranking.partial_cmp(&requested_ranking).unwrap())
            .unwrap_or_else(|e| e);

        // Insert the ranking
        self.rankings
            .insert(pos, Ranking::new(proxy, requested_ranking, weight));
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
}
