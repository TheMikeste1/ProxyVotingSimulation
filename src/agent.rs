use crate::Distribution;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static mut NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default)]
pub struct Agent {
    preference: f64,
    id: usize,
}

impl Agent {
    pub fn new(preference: f64) -> Self {
        unsafe {
            let this_id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            Self {
                preference,
                id: this_id,
            }
        }
    }

    pub fn get_preference(&self) -> f64 {
        self.preference
    }

    pub fn update_preference(
        &mut self,
        extent: f64,
        distribution: &Distribution,
        rng: &mut (impl rand::Rng + ?Sized),
    ) {
        self.preference = distribution.sample(rng, -extent, extent);
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
