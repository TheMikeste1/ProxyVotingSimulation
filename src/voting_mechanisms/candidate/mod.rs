mod instant_runoff_mechanism;
mod median_mechanism;
mod plurality_mechanism;
mod weighted_instant_runoff_mechanism;

mod prelude {
    pub use super::instant_runoff_mechanism::InstantRunoffMechanism;
    pub use super::median_mechanism::MedianMechanism;
    pub use super::plurality_mechanism::PluralityMechanism;
    pub use super::weighted_instant_runoff_mechanism::WeightedInstantRunoffMechanism;
}

pub use prelude::*;
