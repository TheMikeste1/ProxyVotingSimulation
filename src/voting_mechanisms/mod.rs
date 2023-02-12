mod mean_voting_mechanism;
mod median_voting_mechanism;
mod midrange_voting_mechanism;
mod voting_mechanism;

pub mod prelude {
    pub use super::mean_voting_mechanism::*;
    pub use super::median_voting_mechanism::*;
    pub use super::midrange_voting_mechanism::*;
    pub use super::voting_mechanism::*;
}

pub use prelude::*;
