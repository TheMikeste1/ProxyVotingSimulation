use rand_distr::Beta;

mod agent;
mod preference_distribution;
mod ranking;
mod rankings;
mod truth_estimator;

pub mod delegation_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::preference_distribution::*;
    pub use super::ranking::*;
    pub use super::rankings::*;
    pub use super::truth_estimator::*;

    pub use super::delegation_mechanisms;
    pub use super::utils;
    pub use super::voting_mechanisms;
}

pub use prelude::*;

use preference_distribution::PreferenceDistribution;
use voting_mechanisms::average;
use voting_mechanisms::candidate;

fn main() {
    let mut distribution = PreferenceDistribution::from(Beta::new(2.0, 5.0).unwrap());
    let _mech = average::MeanMechanism;
    let _mech2 = candidate::MedianMechanism;
    println!("{}", distribution.generate_value(-1.0, 1.0));
}
