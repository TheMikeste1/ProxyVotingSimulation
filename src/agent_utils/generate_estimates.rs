use crate::{RngLockedDistribution, TruthEstimator};
use rand::rngs::StdRng;
use std::rc::Rc;

pub fn generate_estimates(
    agents: &mut [Rc<dyn TruthEstimator>],
    distribution: &dyn RngLockedDistribution<f64, R = StdRng>,
) {
    for agent in agents {
        let agent = Rc::get_mut(agent).unwrap();
        agent.estimate(distribution);
    }
}
