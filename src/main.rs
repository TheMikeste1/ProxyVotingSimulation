mod agent;
mod data_row;
mod delegation;
mod distribution;
mod proxy;

pub mod coordination_mechanisms;
pub mod utils;
pub mod voting_mechanisms;

pub mod prelude {
    pub use super::agent::*;
    pub use super::data_row::*;
    pub use super::delegation::*;
    pub use super::distribution::*;
    pub use super::proxy::*;

    pub use super::coordination_mechanisms;
    pub use super::utils;
    pub use super::voting_mechanisms;
}

pub use prelude::*;

use std::collections::HashMap;

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
}

fn generate_rows(
    num_agents: usize,
    rows_per_combo: usize,
    coordination_mechanisms: &HashMap<&str, Box<dyn cm::CoordinationMechanism>>,
    voting_mechanisms: HashMap<&str, Box<dyn vm::VotingMechanism>>,
    distributions: HashMap<&str, Distribution>,
) -> Vec<DataRow> {
    todo!("Generate rows")
}
