use crate::Distribution;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static mut NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default)]
pub struct Agent {
    preference: f64,
    shifted_preference: f64,
    id: usize,
}

impl Agent {
    fn new() -> Self {
        unsafe {
            let this_id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            Self {
                preference: 0f64,
                shifted_preference: 0f64,
                id: this_id,
            }
        }
    }

    pub fn new_random(
        extent: f64,
        shift_extent: f64,
        distribution: &Distribution,
        rng: &mut (impl rand::Rng + ?Sized),
    ) -> Self {
        let mut agent = Self::new();
        agent.update_preference(extent, shift_extent, distribution, rng);
        agent
    }

    pub fn get_preference(&self) -> f64 {
        self.preference
    }

    pub fn swap_preference(&mut self) {
        std::mem::swap(&mut self.preference, &mut self.shifted_preference);
    }

    pub fn update_preference(
        &mut self,
        extent: f64,
        shift_extent: f64,
        distribution: &Distribution,
        rng: &mut (impl rand::Rng + ?Sized),
    ) {
        self.preference = distribution.sample(rng, -extent, extent);
        let min_shift = (-extent).max(self.preference - shift_extent);
        let max_shift = extent.min(self.preference + shift_extent);
        self.shifted_preference =
            self.preference + rng.gen_range(min_shift..=max_shift);
    }
}

impl Eq for Agent {}

impl Hash for Agent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.id)
    }
}

impl PartialEq<Self> for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
