use rand::rngs::StdRng;
use rand::SeedableRng;

mod agent;
mod has_id;
mod ranking;
mod rankings;
mod rng_locked_distribution;
mod truth_estimator;

pub mod agent_utils;
pub mod delegation_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::has_id::*;
    pub use super::ranking::*;
    pub use super::rankings::*;
    pub use super::rng_locked_distribution::*;
    pub use super::truth_estimator::*;

    pub use super::agent_utils;
    pub use super::delegation_mechanisms;
    pub use super::utils;
    pub use super::voting_mechanisms;
}

pub use prelude::*;

use crate::delegation_mechanisms::*;
use crate::utils::NamedTuple;
use crate::voting_mechanisms::VotingMechanism;
use voting_mechanisms::average;
use voting_mechanisms::candidate;

fn main() {
    let _distributions: Vec<
        NamedTuple<Box<dyn RngLockedDistribution<f64, R = StdRng>>>,
    > = vec![
        NamedTuple::new(
            "Uniform".into(),
            Box::new(rand_distr::Uniform::new(0.0, 1.0)),
        ),
        NamedTuple::new(
            "Normal".into(),
            Box::new(rand_distr::Normal::new(0.0, 1.0 / 3.0).unwrap()),
        ),
        NamedTuple::new(
            "Beta(0.3, 0.3)".into(),
            Box::new(rand_distr::Beta::new(0.3, 0.3).unwrap()),
        ),
        NamedTuple::new(
            "Beta(50, 50)".into(),
            Box::new(rand_distr::Beta::new(50.0, 50.0).unwrap()),
        ),
        NamedTuple::new(
            "Beta(4, 1)".into(),
            Box::new(rand_distr::Beta::new(4.0, 1.0).unwrap()),
        ),
    ];
    let _rng_factory = Box::new(|_: usize| StdRng::from_entropy());

    let _delegation_mechanisms: Vec<NamedTuple<&dyn DelegationMechanism>> = vec![
        NamedTuple::new("Closest".into(), &ClosestMechanism::new()),
        NamedTuple::new("Closest 2".into(), &ClosestNMechanism::new(2)),
        NamedTuple::new("Closest 3".into(), &ClosestNMechanism::new(3)),
        NamedTuple::new("Closest 5".into(), &ClosestNMechanism::new(5)),
        NamedTuple::new("Closest 10".into(), &ClosestNMechanism::new(10)),
    ];

    let _voting_mechanisms: Vec<NamedTuple<&dyn VotingMechanism>> = vec![
        // Baseline
        NamedTuple::new(
            "Weightless Average All".into(),
            &average::WeightlessAverageAllMechanism,
        ),
        NamedTuple::new(
            "Weightless Average Proxies".into(),
            &average::WeightlessAverageProxiesMechanism,
        ),
        // Average
        NamedTuple::new("Mean".into(), &average::MeanMechanism),
        // Candidate
        NamedTuple::new("Median".into(), &candidate::MedianMechanism),
        NamedTuple::new("Plurality".into(), &candidate::PluralityMechanism),
    ];

    // distribution_factories
}
