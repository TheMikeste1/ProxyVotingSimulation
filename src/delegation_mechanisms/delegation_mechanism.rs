use crate::prelude::{Rankings, TruthEstimator};

pub trait DelegationMechanism {
    fn delegate(&self, proxies: &[&dyn TruthEstimator]) -> Rankings;
}
