use crate::prelude::{Rankings, TruthEstimator};
use crate::voting_mechanisms::VotingMechanism;
use crate::{average_truth_estimators, Truth};
use std::rc::Rc;

pub struct WeightlessAverageAllMechanism;

impl VotingMechanism for WeightlessAverageAllMechanism {
    fn solve(
        &self,
        proxies: &[Rc<dyn TruthEstimator>],
        delegators: &[Rc<dyn TruthEstimator>],
        _rankings: &[Rankings],
    ) -> Truth {
        let proxy_avg = average_truth_estimators!(proxies);
        let delegator_avg = average_truth_estimators!(delegators);
        (proxy_avg + delegator_avg) / 2f64
    }
}
