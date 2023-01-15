use crate::Agent;
use rand::rngs::StdRng;

pub type RngFactory = dyn Fn(usize) -> StdRng;

pub fn generate_agents(n: usize, rng: &RngFactory, extent: f64) -> Vec<Agent> {
    let mut agents = Vec::new();

    for i in 0..n {
        let agent = Agent::new(i as u32, extent, rng(i));
        agents.push(agent);
    }

    agents
}
