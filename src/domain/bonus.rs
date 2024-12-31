use super::{Stats, Tag};

pub struct Bonus {
    pub stat: Stats,
    pub tag: Vec<Tag>,
    pub condition: Vec<Condition>,
    pub formula_to_calc_bonus: String,
}

pub struct Condition {
    pub stat: Stats,
    pub threshold: f64,
    pub formula_to_calc_threshold: String,
}