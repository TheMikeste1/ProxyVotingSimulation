use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, RngCore};
use rand_distr::Beta;
use std::task::ready;

mod agent;
mod degenerified_distribution;
mod has_id;
mod preference_distribution;
mod ranking;
mod rankings;
mod truth_estimator;

pub mod agent_utils;
pub mod delegation_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::degenerified_distribution::*;
    pub use super::has_id::*;
    pub use super::preference_distribution::*;
    pub use super::ranking::*;
    pub use super::rankings::*;
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
use preference_distribution::PreferenceDistribution;
use voting_mechanisms::average;
use voting_mechanisms::candidate;

type DistributionFactory =
    dyn Fn(usize) -> Box<dyn DegenerifiedDistribution<f64, R = ThreadRng>>;

macro_rules! boxed_distribution_factory {
    ($distribution:expr) => {
        Box::new(|_| Box::new($distribution))
    };
}

fn main() {
    let distribution_factories: Vec<NamedTuple<Box<DistributionFactory>>> = vec![
        NamedTuple::new(
            "Uniform".into(),
            boxed_distribution_factory!(rand_distr::Uniform::new(0.0, 1.0)),
        ),
        NamedTuple::new(
            "Normal".into(),
            boxed_distribution_factory!(
                rand_distr::Normal::new(0.0, 1.0 / 3.0).unwrap()
            ),
        ),
        NamedTuple::new(
            "Beta(0.3, 0.3)".into(),
            boxed_distribution_factory!(Beta::new(0.3, 0.3).unwrap()),
        ),
        NamedTuple::new(
            "Beta(50, 50)".into(),
            boxed_distribution_factory!(Beta::new(50.0, 50.0).unwrap()),
        ),
        NamedTuple::new(
            "Beta(4, 1)".into(),
            boxed_distribution_factory!(Beta::new(4.0, 1.0).unwrap()),
        ),
    ];
    let rng_factory = Box::new(|_: usize| thread_rng());

    let delegation_mechanisms: Vec<NamedTuple<&dyn DelegationMechanism>> = vec![
        NamedTuple::new("Closest".into(), &ClosestMechanism::new()),
        NamedTuple::new("Closest 2".into(), &ClosestNMechanism::new(2)),
        NamedTuple::new("Closest 3".into(), &ClosestNMechanism::new(3)),
        NamedTuple::new("Closest 5".into(), &ClosestNMechanism::new(5)),
        NamedTuple::new("Closest 10".into(), &ClosestNMechanism::new(10)),
    ];

    let voting_mechanisms: Vec<NamedTuple<&dyn VotingMechanism>> = vec![
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
