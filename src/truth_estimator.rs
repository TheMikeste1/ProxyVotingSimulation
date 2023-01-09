use crate::HasID;
use ordered_float::OrderedFloat;

pub type Truth = OrderedFloat<f64>;

pub trait TruthEstimator: HasID {
    fn get_last_estimate(&self) -> Option<Truth>;
    fn estimate(&mut self, truth: &Truth) -> Truth;
}
