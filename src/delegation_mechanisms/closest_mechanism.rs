use crate::delegation_mechanisms::{ClosestNMechanism, DelegationMechanism};
use crate::{Rankings, TruthEstimator};

#[derive(Default)]
pub struct ClosestMechanism(ClosestNMechanism);

impl ClosestMechanism {
    pub fn new() -> Self {
        Self(ClosestNMechanism::new(1))
    }
}

impl<'a> DelegationMechanism<'a> for ClosestMechanism {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[&'a dyn TruthEstimator],
    ) -> Rankings<'a> {
        self.0.delegate(agent, proxies)
    }
}
