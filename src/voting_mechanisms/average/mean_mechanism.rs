use crate::prelude::{Rankings, TruthEstimator};
use crate::utils::{average_truth_estimators, sum_rankings_weights};
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;
use std::rc::Rc;

pub struct MeanMechanism;

impl VotingMechanism for MeanMechanism {
    fn solve(
        &mut self,
        _proxies: &[Rc<dyn TruthEstimator>],
        _delegators: &[Rc<dyn TruthEstimator>],
        rankings: &[Rankings],
    ) -> Truth {
        let proxy_weights = sum_rankings_weights(rankings);
        let proxies = proxy_weights
            .iter()
            .map(|pw| pw.proxy.upgrade().expect("Proxy should exist!"))
            .collect::<Vec<_>>();
        let weights = proxy_weights.iter().map(|pw| pw.weight).collect::<Vec<_>>();
        average_truth_estimators(proxies.as_slice(), weights.as_slice())
    }
}
