use crate::{Agent, Truth, TruthEstimator};
use rand::distributions::Distribution;
use rand::Rng;

pub fn generate_estimates<R, D>(truth: Truth, agents: &mut Vec<Agent<R, D>>)
where
    R: Rng,
    D: Distribution<f64>,
{
    for agent in agents {
        agent.estimate(&truth);
    }
}
