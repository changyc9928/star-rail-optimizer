use crate::domain::{Enemy, Path, Relics};
use eyre::Result;

mod acheron;
pub use acheron::{Acheron, AcheronEvaluationTarget};

pub trait Evaluator {
    type Target;

    fn evaluate(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        target: &Self::Target,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64>;
}

pub trait Support: Sync + Send {
    fn get_path(&self) -> Path;
}
