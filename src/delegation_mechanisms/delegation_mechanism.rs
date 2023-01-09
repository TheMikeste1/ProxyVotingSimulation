use crate::prelude::{Rankings, TruthEstimator};
use std::rc::Rc;

pub trait DelegationMechanism {
    fn delegate(
        &self,
        agent: &dyn TruthEstimator,
        proxies: &[Rc<dyn TruthEstimator>],
    ) -> Rankings;
}
