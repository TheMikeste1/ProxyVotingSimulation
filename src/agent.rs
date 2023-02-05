use rand::distributions::Distribution;

#[derive(Debug, Default)]
pub struct Agent {
    preference: f64,
}

impl Agent {
    pub fn new(preference: f64) -> Self {
        Self { preference }
    }

    pub fn get_preference(&self) -> f64 {
        self.preference
    }

    pub fn update_preference(
        &mut self,
        extent: f64,
        distribution: &impl Distribution<f64>,
        rng: &mut (impl rand::Rng + ?Sized),
    ) {
        self.preference = distribution.sample(rng) * 2f64 + (-extent);
    }
}
