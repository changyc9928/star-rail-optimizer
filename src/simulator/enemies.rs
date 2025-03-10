use std::sync::{Arc, Mutex};

use super::action_bar::Unit;

pub trait EnemyTrait: Unit {
    fn get_enemy_id(&self) -> i64;

    fn get_unit_trait_object(&self) -> Arc<Mutex<dyn Unit>>;
}

pub struct Enemy {
    pub level: u8,
}
