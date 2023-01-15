use rand::distributions::Distribution;
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Beta;

mod agent;
pub mod has_id;
mod preference_distribution;
mod ranking;
mod rankings;
mod truth_estimator;

pub mod delegation_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::has_id::*;
    pub use super::preference_distribution::*;
    pub use super::ranking::*;
    pub use super::rankings::*;

    pub use super::delegation_mechanisms;
    pub use super::truth_estimator::*;
    pub use super::utils;
    pub use super::voting_mechanisms;
}

pub use prelude::*;

use preference_distribution::PreferenceDistribution;
use voting_mechanisms::average;
use voting_mechanisms::candidate;

fn main() {
    let mut distribution: PreferenceDistribution<ThreadRng, _> =
        PreferenceDistribution::from(Beta::new(2.0, 5.0).unwrap());
    let _mech = average::MeanMechanism;
    let _mech2 = candidate::MedianMechanism;
    println!("{}", distribution.generate_value(-1.0, 1.0));
}

fn generate_agents<R, D>(
    n: usize,
    distribution: &dyn Fn() -> D,
    rng: &dyn Fn() -> R,
    extent: f64,
) -> Vec<Agent<R, D>>
where
    R: Rng,
    D: Distribution<f64>,
{
    let mut agents = Vec::new();

    for i in 0..n {
        let agent = Agent::new(
            i as u32,
            extent,
            PreferenceDistribution::new(distribution(), rng()),
        );
        agents.push(agent);
    }

    agents
}
