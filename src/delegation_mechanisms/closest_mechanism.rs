use crate::delegation_mechanisms::{ClosestNMechanism, DelegationMechanism};
use crate::{Rankings, TruthEstimator};
use std::rc::Rc;

#[derive(Default)]
pub struct ClosestMechanism(ClosestNMechanism);

impl ClosestMechanism {
    pub fn new() -> Self {
        Self(ClosestNMechanism::new(1))
    }
}

impl DelegationMechanism for ClosestMechanism {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[Rc<dyn TruthEstimator>],
    ) -> Rankings {
        self.0.delegate(agent, proxies)
    }
}
