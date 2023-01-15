use crate::prelude::{Rankings, TruthEstimator};
use crate::utils::{sum_rankings_weights, ProxyWeightSum};
use crate::voting_mechanisms::VotingMechanism;
use crate::{Truth, Weight};
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct MedianMechanism;

impl VotingMechanism for MedianMechanism {
    fn solve(
        &mut self,
        _proxies: &[Rc<dyn TruthEstimator>],
        _delegators: &[Rc<dyn TruthEstimator>],
        rankings: &[Rankings],
    ) -> Truth {
        let proxy_weights = sum_rankings_weights(rankings);
        let total_weight = proxy_weights.iter().map(|r| r.weight).sum::<Weight>();

        let median_weight = total_weight / 2.0;
        let mut current_weight: Weight = OrderedFloat(0.0);
        // Find the median value
        for (i, ProxyWeightSum { proxy, weight }) in proxy_weights.iter().enumerate() {
            current_weight += weight;
            if current_weight >= median_weight {
                return proxy.get_last_estimate().unwrap();
            }
        }
        unreachable!(
            "The median weight should always be found. \
        Is the total weight more than the sum of the weights? \
        Median weight: {}, total weight: {}",
            median_weight, total_weight
        );
    }
}
