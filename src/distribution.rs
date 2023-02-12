use rand::Rng;
use rand_distr::num_traits::clamp;
use rand_distr::{Beta, Normal, Uniform};

pub enum Distribution {
    Uniform,
    Normal,
    Beta { alpha: f64, beta: f64 },
}

impl Distribution {
    pub fn sample(&self, rng: &mut (impl Rng + ?Sized), min: f64, max: f64) -> f64 {
        match self {
            Distribution::Uniform => rng.sample(Uniform::new(min, max)),
            Distribution::Normal => normal_in_range(rng, min, max),
            Distribution::Beta { alpha, beta } => {
                beta_in_range(rng, min, max, *alpha, *beta)
            }
        }
    }
}

fn beta_in_range(
    rng: &mut (impl Rng + ?Sized),
    min: f64,
    max: f64,
    alpha: f64,
    beta: f64,
) -> f64 {
    let intermediate = rng.sample(Beta::new(alpha, beta).unwrap());
    intermediate * (max - min) + min
}

fn normal_in_range(rng: &mut (impl Rng + ?Sized), min: f64, max: f64) -> f64 {
    let mean = (max + min) / 2f64;
    let std_dev = (max - min) / 6f64;
    clamp(rng.sample(Normal::new(mean, std_dev).unwrap()), min, max)
}
