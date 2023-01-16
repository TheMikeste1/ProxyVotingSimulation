use crate::prelude::{Rankings, TruthEstimator};
use crate::utils::sum_rankings_weights;
use crate::voting_mechanisms::VotingMechanism;
use crate::Truth;
use std::borrow::Borrow;
use std::rc::Rc;

pub struct PluralityMechanism;

impl VotingMechanism for PluralityMechanism {
    fn solve(
        &self,
        _proxies: &[Rc<dyn TruthEstimator>],
        _delegators: &[Rc<dyn TruthEstimator>],
        rankings: &[Rankings],
    ) -> Truth {
        let summed_rankings = sum_rankings_weights(rankings);
        summed_rankings
            .iter()
            .max_by(|a, b| a.weight.partial_cmp(b.weight.borrow()).unwrap())
            .unwrap()
            .proxy
            .upgrade()
            .expect("Proxy should exist!")
            .get_last_estimate()
            .unwrap()
    }
}
