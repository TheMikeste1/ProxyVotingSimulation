use crate::{Agent, TruthEstimator};
use rand::rngs::StdRng;
use std::rc::Rc;

pub type RngFactory = dyn Fn(usize) -> StdRng;

pub fn generate_agents(
    n: usize,
    rng: &RngFactory,
    extent: f64,
) -> Vec<Rc<dyn TruthEstimator>> {
    let mut agents = Vec::new();

    for i in 0..n {
        let agent = Rc::new(Agent::new(i as u32, extent, rng(i)));
        agents.push(agent as Rc<dyn TruthEstimator>);
    }

    agents
}
