use crate::{Agent, RngLockedDistribution, TruthEstimator};
use rand::rngs::StdRng;
use rand::Rng;

pub fn generate_estimates<R, D>(
    agents: &mut Vec<Agent>,
    distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
) where
    R: Rng,
{
    for agent in agents {
        agent.estimate(distribution);
    }
}
