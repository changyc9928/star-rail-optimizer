use crate::domain::{BattleConditionEnum, Enemy, Relics};
use eyre::Result;

mod acheron;
pub use acheron::Acheron;

pub trait Evaluator {
    fn evaluate(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        target: &str,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64>;
}
