pub mod average;
pub mod candidate;

mod voting_mechanism;

mod prelude {
    pub use super::average;
    pub use super::candidate;

    pub use super::voting_mechanism::VotingMechanism;
}

pub use prelude::*;
