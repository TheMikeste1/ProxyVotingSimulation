use crate::delegation_mechanisms::DelegationMechanism;
use crate::{Rankings, TruthEstimator};

#[derive(Default)]
pub struct ClosestMechanism;

impl ClosestMechanism {
    fn new() -> ClosestMechanism {
        ClosestMechanism
    }
}

impl DelegationMechanism for ClosestMechanism {
    fn delegate(&self, proxies: &[&dyn TruthEstimator]) -> Rankings {
        todo!("Implement ClosestMechanism::delegate")
    }
}
