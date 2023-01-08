use rand_distr::Beta;

mod agent;
mod preference_distribution;
mod ranking;
mod rankings;
mod truth_estimator;

pub mod delegation_mechanisms;
pub mod voting_mechanisms;

pub mod prelude {
    pub use crate::agent::*;
    pub use crate::preference_distribution::*;
    pub use crate::ranking::*;
    pub use crate::rankings::*;
    pub use crate::truth_estimator::*;

    pub use crate::delegation_mechanisms;
    pub use crate::voting_mechanisms;
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
