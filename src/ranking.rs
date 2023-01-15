use crate::TruthEstimator;
use ordered_float::OrderedFloat;
use std::rc::Rc;

pub type Weight = OrderedFloat<f64>;

#[derive(Clone)]
pub struct Ranking {
    pub proxy: Rc<dyn TruthEstimator>,
    /// The ranking requested by the proxy
    pub requested_ranking: u32,
    pub weight: Weight,
}

impl Ranking {
    pub fn new(
        proxy: Rc<dyn TruthEstimator>,
        requested_ranking: u32,
        weight: Weight,
    ) -> Self {
        // Weight must be positive or zero
        assert!(weight >= OrderedFloat(0.0));

        let proxy = Rc::clone(&proxy);
        Self {
            proxy,
            requested_ranking,
            weight,
        }
    }
}
