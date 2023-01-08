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

impl DelegationMechanism for ClosestNMechanism {
    fn delegate(&self, proxies: &[&dyn TruthEstimator]) -> Rankings {
        todo!("Implement ClosestNMechanism::delegate")
    }
}

impl Default for ClosestNMechanism {
    fn default() -> Self {
        Self::new(1)
    }
}
