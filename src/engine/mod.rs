use std::collections::HashMap;

use crate::domain::Stats;

// pub mod evaluator;
pub mod optimizer;
pub mod simulated_annealing;

pub type StatBonusMap = HashMap<Stats, f64>;
