extern crate crossbeam;

use crossbeam::thread;
use indicatif::{MultiProgress, ProgressBar};
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::rc::Rc;
use std::sync::Arc;

mod agent;
mod data_row;
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
    pub use super::data_row::*;
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

use crate::agent_utils::{generate_agents, generate_estimates};
use crate::delegation_mechanisms::*;
use crate::utils::{save_to_file, sum_rankings_weights, NamedTuple};
use crate::voting_mechanisms::VotingMechanism;
use voting_mechanisms::average;
use voting_mechanisms::candidate;

fn main() {
    let distributions: Vec<
        NamedTuple<Box<dyn RngLockedDistribution<f64, R = StdRng> + Sync>>,
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

    let delegation_mechanisms: Vec<NamedTuple<Box<dyn DelegationMechanism + Sync>>> = vec![
        NamedTuple::new("Closest".into(), Box::new(ClosestMechanism::new())),
        NamedTuple::new("Closest 2".into(), Box::new(ClosestNMechanism::new(2))),
        NamedTuple::new("Closest 3".into(), Box::new(ClosestNMechanism::new(3))),
        NamedTuple::new("Closest 5".into(), Box::new(ClosestNMechanism::new(5))),
        NamedTuple::new("Closest 10".into(), Box::new(ClosestNMechanism::new(10))),
    ];

    let voting_mechanisms: Vec<NamedTuple<Box<dyn VotingMechanism + Sync>>> = vec![
        // Baseline
        NamedTuple::new(
            "Weightless Average All".into(),
            Box::new(average::WeightlessAverageAllMechanism),
        ),
        NamedTuple::new(
            "Weightless Average Proxies".into(),
            Box::new(average::WeightlessAverageProxiesMechanism),
        ),
        // Average
        NamedTuple::new("Mean".into(), Box::new(average::MeanMechanism)),
        // Candidate
        NamedTuple::new("Median".into(), Box::new(candidate::MedianMechanism)),
        NamedTuple::new("Plurality".into(), Box::new(candidate::PluralityMechanism)),
    ];

    let data =
        perform_experiments(distributions, delegation_mechanisms, voting_mechanisms);
    save_to_file(data);
}

fn perform_experiments(
    distributions: Vec<
        NamedTuple<Box<dyn RngLockedDistribution<f64, R = StdRng> + Sync>>,
    >,
    delegation_mechanisms: Vec<NamedTuple<Box<dyn DelegationMechanism + Sync>>>,
    voting_mechanisms: Vec<NamedTuple<Box<dyn VotingMechanism + Sync>>>,
) -> Vec<DataRow> {
    let rng_factory = |_: usize| StdRng::from_entropy();
    let num_threads = 8;
    let num_agents = 435; // 435 is the number of members of the US House of Representatives

    let total_combinations =
        distributions.len() * delegation_mechanisms.len() * voting_mechanisms.len();
    let items_per_thread =
        (total_combinations as f64 / num_threads as f64).ceil() as usize;

    let chunks = distributions
        .iter()
        .cartesian_product(delegation_mechanisms.iter())
        .cartesian_product(voting_mechanisms.iter())
        .chunks(items_per_thread);

    let multi_progress = Arc::new(MultiProgress::new());
    multi_progress
        .println("Starting experiments...")
        .expect("Failed to print to progress bar");

    let mut all_rows = Vec::new();
    thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in &chunks {
            let chunk = chunk.collect::<Vec<_>>();

            let progress_bar = multi_progress.add(ProgressBar::new(chunk.len() as u64));
            progress_bar.set_style(
                indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("##-")
            );
            progress_bar.tick();
            let thread = s.spawn(move |_| -> Vec<DataRow> {
                let mut data = Vec::with_capacity(chunk.len() * (num_agents - 2));
                // Perform the experiments for each combination
                // Initialize new agents for each thread
                let mut agents = generate_agents(num_agents, &rng_factory, 1.0);
                for ((distribution, delegation_mechanism), voting_mechanism) in &chunk {
                    let rows = perform_experiment(
                        &mut agents,
                        distribution,
                        delegation_mechanism,
                        voting_mechanism,
                        &progress_bar
                    );
                    data.extend(rows);
                    progress_bar.inc(1);
                }
                progress_bar.finish();
                data
            });
            handles.push(thread);
        }
        for thread in handles {
            let rows = thread.join().unwrap();
            all_rows.extend(rows);
        }
    })
    .expect("Thread panicked");

    all_rows
}

fn perform_experiment(
    agents: &mut Vec<Rc<dyn TruthEstimator>>,
    distribution: &NamedTuple<Box<dyn RngLockedDistribution<f64, R = StdRng> + Sync>>,
    delegation_mechanism: &NamedTuple<Box<dyn DelegationMechanism + Sync>>,
    voting_mechanism: &NamedTuple<Box<dyn VotingMechanism + Sync>>,
    progress_bar: &ProgressBar,
) -> Vec<DataRow> {
    generate_estimates(agents, distribution.value.as_ref());

    // Make a vector of length agents.len() - 2.
    // We subtract 2 since we will never have 0 proxies nor all proxies
    let mut data = Vec::with_capacity(agents.len() - 2);
    // Start with n - 1 proxies, go down to 1 proxy
    for num_proxies in (1..agents.len()).rev() {
        let proxies = &agents[0..num_proxies];
        let inactive_agents = &agents[num_proxies..];

        // Perform delegations
        let rankings = inactive_agents
            .iter()
            .map(|a| delegation_mechanism.value.delegate(a, proxies))
            .collect::<Vec<_>>();

        // Perform voting
        let out = voting_mechanism
            .value
            .solve(proxies, inactive_agents, &rankings);

        let proxy_weights = sum_rankings_weights(&rankings)
            .iter()
            .map(|w| w.weight)
            .collect::<Vec<_>>();
        let min_proxy_weight = proxy_weights.iter().min().unwrap();
        let max_proxy_weight = proxy_weights.iter().max().unwrap();
        let avg_proxy_weight =
            proxy_weights.iter().sum::<Weight>() / proxy_weights.len() as f64;

        let row = DataRow {
            distribution: distribution.name.clone(),
            voting_mechanism: voting_mechanism.name.clone(),
            number_of_proxies: num_proxies as u32,
            number_of_delegates: (agents.len() - num_proxies) as u32,
            estimate: *out,
            min_proxy_weight: **min_proxy_weight,
            max_proxy_weight: **max_proxy_weight,
            average_proxy_weight: *avg_proxy_weight,
        };
        data.push(row);

        progress_bar.tick();
    }
    data
}
