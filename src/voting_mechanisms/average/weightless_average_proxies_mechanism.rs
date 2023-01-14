use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::{average_truth_estimators, Truth};
use std::rc::Rc;

pub struct WeightlessAverageProxiesMechanism;

impl VotingMechanism for WeightlessAverageProxiesMechanism {
    fn solve(
        &mut self,
        proxies: &[Rc<dyn TruthEstimator>],
        _delegators: &[Rc<dyn TruthEstimator>],
        _rankings: &[Rankings],
    ) -> Truth {
        average_truth_estimators!(proxies)
    }
}
