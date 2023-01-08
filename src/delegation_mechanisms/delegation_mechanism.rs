use crate::prelude::{Rankings, TruthEstimator};

pub trait DelegationMechanism<'a> {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[&'a dyn TruthEstimator],
    ) -> Rankings<'a>;
}
