use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;
use std::rc::Rc;

pub struct MedianMechanism;

impl VotingMechanism for MedianMechanism {
    fn solve(
        &mut self,
        _proxies: &[Rc<dyn TruthEstimator>],
        _delegators: &[Rc<dyn TruthEstimator>],
        _rankings: &[Rankings],
    ) -> Truth {
        todo!("Implement MedianMechanism::solve")
    }
}
