use crate::delegation_mechanisms::DelegationMechanism;
use crate::{Rankings, TruthEstimator};

pub struct ClosestNMechanism {
    n: u32,
}

impl ClosestNMechanism {
    fn new(n: u32) -> ClosestNMechanism {
        ClosestNMechanism { n }
    }
}

impl<'a> DelegationMechanism<'a> for ClosestNMechanism {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[&'a dyn TruthEstimator],
    ) -> Rankings<'a> {
        todo!("Implement ClosestNMechanism::delegate")
    }
}

impl Default for ClosestNMechanism {
    fn default() -> Self {
        Self::new(1)
    }
}
