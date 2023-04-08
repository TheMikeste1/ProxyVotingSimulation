use crate::Distribution;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static mut NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default)]
pub struct Agent {
    preference: f64,
    shifted_preference: f64,
    id: usize,
    shifted: bool,
}

impl Agent {
    fn new() -> Self {
        unsafe {
            let this_id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            Self {
                preference: 0f64,
                shifted_preference: 0f64,
                id: this_id,
                shifted: false,
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

    pub fn get_current_preference(&self) -> f64 {
        if self.shifted {
            self.shifted_preference
        } else {
            self.preference
        }
    }

    pub fn get_shifted_preference(&self) -> f64 {
        self.shifted_preference
    }

    pub fn get_original_preference(&self) -> f64 {
        self.preference
    }

    pub fn distance_to_proxy(&self, proxy: &Self) -> f64 {
        // Delegators only know the proxy's original preference
        (self.preference - proxy.get_original_preference()).abs()
    }

    pub fn swap_preference(&mut self) {
        self.shifted = !self.shifted;
    }

    pub fn update_preference(
        &mut self,
        extent: f64,
        _shift_extent: f64,
        distribution: &Distribution,
        rng: &mut (impl rand::Rng + ?Sized),
    ) {
        self.preference = distribution.sample(rng, -extent, extent);
        let min_shift = (-extent).max(self.preference - _shift_extent);
        let max_shift = extent.min(self.preference + _shift_extent);
        self.shifted_preference = self.preference + rng.gen_range(min_shift..=max_shift); //distribution.sample(rng, -extent, extent);
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
