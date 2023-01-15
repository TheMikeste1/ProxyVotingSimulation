use crate::Truth;
use rand::Rng;
use rand_distr::Distribution;

pub struct PreferenceDistribution<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    rng: R,
    distribution: D,
}

impl<R, D> PreferenceDistribution<R, D>
where
    R: Rng,
    D: Distribution<f64>,
{
    pub fn new(distribution: D, rng: R) -> Self {
        Self { rng, distribution }
    }

    pub fn generate_value(&mut self, min: f64, max: f64) -> Truth {
        let value = self.distribution.sample(&mut self.rng);
        Truth::from(min + (max - min) * value)
    }
}

impl<R, D> From<D> for PreferenceDistribution<R, D>
where
    R: Rng + Default,
    D: Distribution<f64>,
{
    fn from(distribution: D) -> Self {
        Self::new(distribution, R::default())
    }
}
