use crate::Truth;
use rand::RngCore;
use rand_distr::Distribution;

pub struct PreferenceDistribution<'a, D>
where
    D: Distribution<f64>,
{
    rng: Box<dyn RngCore + 'a>,
    distribution: D,
}

impl<'a, D> PreferenceDistribution<'a, D>
where
    D: Distribution<f64>,
{
    pub fn new(distribution: D, rng: impl RngCore + 'a) -> Self {
        let rng = Box::new(rng);
        Self { rng, distribution }
    }

    pub fn generate_value(&mut self, min: f64, max: f64) -> Truth {
        let value = self.distribution.sample(&mut *self.rng);
        Truth::from(min + (max - min) * value)
    }
}

impl<D> From<D> for PreferenceDistribution<'_, D>
where
    D: Distribution<f64>,
{
    fn from(distribution: D) -> Self {
        Self::new(distribution, rand::thread_rng())
    }
}
