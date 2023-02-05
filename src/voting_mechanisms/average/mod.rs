mod instant_runoff_mechanism;
mod mean_mechanism;
mod ranked_choice_mechanism;
mod weighted_instant_runoff_mechanism;
mod weightless_average_all_mechanism;
mod weightless_average_proxies_mechanism;

mod prelude {
    pub use super::instant_runoff_mechanism::InstantRunoffMechanism;
    pub use super::mean_mechanism::MeanMechanism;
    pub use super::ranked_choice_mechanism::RankedChoiceMechanism;
    pub use super::weighted_instant_runoff_mechanism::WeightedInstantRunoffMechanism;
    pub use super::weightless_average_all_mechanism::WeightlessAverageAllMechanism;
    pub use super::weightless_average_proxies_mechanism::WeightlessAverageProxiesMechanism;
}

pub use prelude::*;
