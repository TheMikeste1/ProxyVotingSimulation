use crate::prelude::{Rankings, TruthEstimator};
use crate::Truth;
use std::rc::Rc;

pub trait VotingMechanism {
    fn solve(
        &mut self,
        proxies: &[Rc<dyn TruthEstimator>],
        delegators: &[Rc<dyn TruthEstimator>],
        rankings: &[Rankings],
    ) -> Truth;
}
