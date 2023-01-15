use crate::preference_distribution::PreferenceDistribution;
use crate::Agent;
use rand::distributions::Distribution;
use rand::Rng;

pub type DistributionFactory<D> = dyn Fn(usize) -> D;
pub type RngFactory<R> = dyn Fn(usize) -> R;

pub fn generate_agents<R, D>(
    n: usize,
    distribution: &DistributionFactory<D>,
    rng: &RngFactory<R>,
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
            PreferenceDistribution::new(distribution(i), rng(i)),
        );
        agents.push(agent);
    }

    agents
}
