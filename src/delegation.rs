use std::rc::Rc;

use crate::Agent;

#[derive(Debug)]
pub struct Delegation {
    constituent: Rc<Agent>,
    rank: u32,
    weight: f64,
}

impl Delegation {
    pub fn new(constituent: Rc<Agent>, rank: u32, weight: f64) -> Self {
        Self {
            constituent,
            rank,
            weight,
        }
    }

    pub fn get_constituent(&self) -> &Agent {
        &self.constituent
    }

    pub fn get_rank(&self) -> u32 {
        self.rank
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }
}
