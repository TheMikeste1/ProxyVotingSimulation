mod agent;
mod data_row;
mod distribution;

pub mod coordination_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::data_row::*;
    pub use super::distribution::*;

    pub use super::coordination_mechanisms;
    pub use super::utils;
    pub use super::voting_mechanisms;
}

pub use prelude::*;

use indicatif::ProgressBar;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{SeedableRng};
use std::collections::HashMap;

use crate::utils::save_to_file;
use crate::vm::WeightedVote;
use coordination_mechanisms as cm;
use voting_mechanisms as vm;

fn main() {
    let mut coordination_mechanisms =
        HashMap::<&str, Box<dyn cm::CoordinationMechanism>>::new();
    coordination_mechanisms
        .insert("Expert", Box::<cm::ExpertCoordinationMechanism>::default());
    coordination_mechanisms
        .insert("Mean", Box::<cm::MeanCoordinationMechanism>::default());
    coordination_mechanisms
        .insert("Median", Box::<cm::MedianCoordinationMechanism>::default());
    let coordination_mechanisms = coordination_mechanisms;

    let mut voting_mechanisms = HashMap::<&str, Box<dyn vm::VotingMechanism>>::new();
    voting_mechanisms.insert("Mean", Box::<vm::MeanVotingMechanism>::default());
    voting_mechanisms.insert("Median", Box::<vm::MedianVotingMechanism>::default());
    voting_mechanisms.insert("Midrange", Box::<vm::MidrangeVotingMechanism>::default());
    let voting_mechanisms = voting_mechanisms;

    let distributions = HashMap::<&str, Distribution>::from([
        ("Uniform", Distribution::Uniform),
        ("Normal", Distribution::Normal),
        (
            "Beta(0.3, 0.3)",
            Distribution::Beta {
                alpha: 0.3,
                beta: 0.3,
            },
        ),
        (
            "Beta(50, 50)",
            Distribution::Beta {
                alpha: 50.0,
                beta: 50.0,
            },
        ),
        (
            "Beta(4, 1)",
            Distribution::Beta {
                alpha: 4.0,
                beta: 1.0,
            },
        ),
    ]);

    let seed = 5372052; //rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);
    let num_agents = 24;
    let rows_per_combo = 1024;
    let shift = 0.2;
    let proxy_weight = 1f64;
    let constituent_weight = 1f64;
    let rows = generate_rows(
        num_agents,
        shift,
        rows_per_combo,
        &coordination_mechanisms,
        &voting_mechanisms,
        &distributions,
        &mut rng,
        proxy_weight,
        constituent_weight
    );
    save_to_file(
        &format!("{}_shift-{}_agents-{}_weight_p{}_c_{}", seed, shift, num_agents, proxy_weight, constituent_weight),
        rows,
    );
}

fn generate_rows(
    num_agents: usize,
    shift_extent: f64,
    rows_per_combo: usize,
    coordination_mechanisms: &HashMap<&str, Box<dyn cm::CoordinationMechanism>>,
    voting_mechanisms: &HashMap<&str, Box<dyn vm::VotingMechanism>>,
    distributions: &HashMap<&str, Distribution>,
    rng: &mut (impl rand::Rng + ?Sized),
    proxy_weight: f64,
    constituent_weight: f64,
) -> Vec<DataRow> {
    let ids = (0..rows_per_combo).collect_vec();
    let generations = distributions
        .iter()
        .cartesian_product(ids.iter())
        .map(|(d, r)| (d, r));

    let total_combos = (num_agents - 2)
        * ids.len()
        * (coordination_mechanisms.len() + 2)  // +2 for All Agents and Active Only
        * 2 // x2 for candidate mechanisms
        * voting_mechanisms.len()
        * distributions.len()
        * 2; // x2 for shifted and unshifted
    let progress_bar = ProgressBar::new(total_combos as u64);
    progress_bar.println("Starting experiments...");
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("##-")
    );
    progress_bar.tick();

    let mut rows = Vec::new();
    for (distribution, id) in generations {
        let dist_name = distribution.0.to_string();
        let mut agents = (0..num_agents)
            .map(|_| Agent::new_random(1.0, shift_extent, distribution.1, rng))
            .collect_vec();

        // Preshift
        run_for_agents(
            *id as u32,
            &progress_bar,
            &mut rows,
            num_agents,
            agents.as_slice(),
            voting_mechanisms,
            coordination_mechanisms,
            dist_name.clone(),
            false,
            proxy_weight,
            constituent_weight
        );

        // Shift
        agents.iter_mut().for_each(|a| a.swap_preference());

        // Postshift
        run_for_agents(
            *id as u32,
            &progress_bar,
            &mut rows,
            num_agents,
            agents.as_slice(),
            voting_mechanisms,
            coordination_mechanisms,
            dist_name,
            true,
            proxy_weight,
            constituent_weight
        );
    }
    rows
}

#[allow(clippy::too_many_arguments)]
fn run_for_agents<'a>(
    run_id: u32,
    progress_bar: &ProgressBar,
    rows: &mut Vec<DataRow>,
    num_agents: usize,
    agents: &'a [Agent],
    voting_mechanisms: &'a HashMap<&str, Box<dyn vm::VotingMechanism>>,
    coordination_mechanisms: &'a HashMap<&str, Box<dyn cm::CoordinationMechanism>>,
    dist_name: String,
    shifted: bool,
    proxy_weight: f64,
    constituent_weight: f64,
) {
    for num_proxies in 1..num_agents {
        let proxies = agents.iter().take(num_proxies).collect_vec();
        let delegators = agents.iter().skip(num_proxies).collect_vec();
        let delegation_map = select_delegates(&proxies, &delegators);

        for (vm_name, vm) in voting_mechanisms {
            let vm_name = vm_name.to_string();
            let estimate =
                vote_no_delegations(agents.iter().collect_vec().as_slice(), &**vm);
            rows.push(DataRow {
                generation_id: run_id,
                distribution: dist_name.clone(),
                coordination_mechanism: "All Agents".to_string(),
                voting_mechanism: vm_name.clone(),
                discrete_vote: false,
                number_of_delegators: (num_agents - num_proxies) as u32,
                number_of_proxies: num_proxies as u32,
                estimate,
                min_proxy_weight: 1f64,
                max_proxy_weight: 1f64,
                average_proxy_weight: 1f64,
                median_proxy_weight: 1f64,
                shifted,
            });
            progress_bar.inc(1);

            let estimate = vote_candidate_no_delegations(
                agents.iter().collect_vec().as_slice(),
                &**vm,
            );
            rows.push(DataRow {
                generation_id: run_id,
                distribution: dist_name.clone(),
                coordination_mechanism: "All Agents".to_string(),
                voting_mechanism: vm_name.clone(),
                discrete_vote: true,
                number_of_delegators: (num_agents - num_proxies) as u32,
                number_of_proxies: num_proxies as u32,
                estimate,
                min_proxy_weight: 1f64,
                max_proxy_weight: 1f64,
                average_proxy_weight: 1f64,
                median_proxy_weight: 1f64,
                shifted,
            });
            progress_bar.inc(1);

            let estimate = vote_no_delegations(proxies.as_slice(), &**vm);
            rows.push(DataRow {
                generation_id: run_id,
                distribution: dist_name.clone(),
                coordination_mechanism: "Active Only".to_string(),
                voting_mechanism: vm_name.clone(),
                discrete_vote: false,
                number_of_delegators: (num_agents - num_proxies) as u32,
                number_of_proxies: num_proxies as u32,
                estimate,
                min_proxy_weight: 1f64,
                max_proxy_weight: 1f64,
                average_proxy_weight: 1f64,
                median_proxy_weight: 1f64,
                shifted,
            });
            progress_bar.inc(1);

            let estimate = vote_candidate_no_delegations(proxies.as_slice(), &**vm);
            rows.push(DataRow {
                generation_id: run_id,
                distribution: dist_name.clone(),
                coordination_mechanism: "Active Only".to_string(),
                voting_mechanism: vm_name.clone(),
                discrete_vote: true,
                number_of_delegators: (num_agents - num_proxies) as u32,
                number_of_proxies: num_proxies as u32,
                estimate,
                min_proxy_weight: 1f64,
                max_proxy_weight: 1f64,
                average_proxy_weight: 1f64,
                median_proxy_weight: 1f64,
                shifted,
            });
            progress_bar.inc(1);

            for (cm_name, cm) in coordination_mechanisms {
                let cm_name = cm_name.to_string();
                let delegations = delegation_map
                    .iter()
                    .map(|(p, d)| {
                        let vote = cm.coordinate(p, d.as_slice(), proxy_weight, constituent_weight);
                        let weight = proxy_weight + constituent_weight * d.len() as f64;
                        WeightedVote { vote, weight }
                    })
                    .collect_vec();
                let min_weight = delegations
                    .iter()
                    .map(|d| d.weight)
                    .min_by(|w1, w2| w1.partial_cmp(w2).unwrap())
                    .unwrap();
                let max_weight = delegations
                    .iter()
                    .map(|d| d.weight)
                    .max_by(|w1, w2| w1.partial_cmp(w2).unwrap())
                    .unwrap();
                let average_weight = delegations.iter().map(|d| d.weight).sum::<f64>()
                    / num_proxies as f64;
                let median_weight = delegations
                    .iter()
                    .map(|d| d.weight)
                    .sorted_by(|w1, w2| w1.partial_cmp(w2).unwrap())
                    .nth(num_proxies / 2)
                    .unwrap();

                let estimate = vm.vote(delegations.as_slice());
                rows.push(DataRow {
                    generation_id: run_id,
                    distribution: dist_name.clone(),
                    coordination_mechanism: cm_name.clone(),
                    voting_mechanism: vm_name.clone(),
                    discrete_vote: false,
                    number_of_delegators: (num_agents - num_proxies) as u32,
                    number_of_proxies: num_proxies as u32,
                    estimate,
                    min_proxy_weight: min_weight,
                    max_proxy_weight: max_weight,
                    average_proxy_weight: average_weight,
                    median_proxy_weight: median_weight,
                    shifted,
                });

                let estimate = vm.vote(
                    delegations
                        .iter()
                        .map(|p| WeightedVote {
                            vote: p.vote.round(),
                            weight: p.weight,
                        })
                        .collect_vec()
                        .as_slice(),
                );
                rows.push(DataRow {
                    generation_id: run_id,
                    distribution: dist_name.clone(),
                    coordination_mechanism: cm_name.clone(),
                    voting_mechanism: vm_name.clone(),
                    discrete_vote: true,
                    number_of_delegators: (num_agents - num_proxies) as u32,
                    number_of_proxies: num_proxies as u32,
                    estimate,
                    min_proxy_weight: min_weight,
                    max_proxy_weight: max_weight,
                    average_proxy_weight: average_weight,
                    median_proxy_weight: median_weight,
                    shifted,
                });
                progress_bar.inc(1);
            }
        }
    }
}

fn select_delegates<'a>(
    proxies: &[&'a Agent],
    delegators: &[&'a Agent]
) -> HashMap<&'a Agent, Vec<&'a Agent>> {
    let mut delegation_map =
        HashMap::<&Agent, Vec<&Agent>>::from_iter(proxies.iter().map(|&p| (p, vec![])));
    assert_eq!(delegation_map.len(), proxies.len());
    // Choose proxies
    for delegator in delegators.iter() {
        let (chosen, _) = proxies
            .iter()
            .map(|&p| (p, delegator.distance_to_proxy(p)))
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();
        delegation_map.get_mut(chosen).unwrap().push(delegator);
    }
    delegation_map
}

fn vote_no_delegations(
    agents: &[&Agent],
    voting_mechanism: &dyn vm::VotingMechanism,
) -> f64 {
    let delegations = agents
        .iter()
        .map(|p| WeightedVote {
            vote: p.get_current_preference(),
            weight: 1f64,
        })
        .collect_vec();
    voting_mechanism.vote(delegations.as_slice())
}

fn vote_candidate_no_delegations(
    agents: &[&Agent],
    voting_mechanism: &dyn vm::VotingMechanism,
) -> f64 {
    let delegations = agents
        .iter()
        .map(|p| WeightedVote {
            vote: p.get_current_preference().round(),
            weight: 1f64,
        })
        .collect_vec();
    voting_mechanism.vote(delegations.as_slice())
}
