use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BattleConditionEnum, Stats, Tag};

#[derive(Clone, Debug)]
pub struct LightConeEntity {
    pub base_hp: f64,
    pub base_atk: f64,
    pub base_def: f64,
    pub _light_cone: LightCone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub ascension: u8,
    pub superimposition: u8,
    pub location: Option<String>,
    pub lock: bool,
    pub _uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LightConeStats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
}

impl LightConeEntity {
    pub fn get_bonus(
        &self,
        tags: &[Tag],
        battle_conditions: &[BattleConditionEnum],
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        match self._light_cone.id.as_str() {
            "21001" => {
                let mut stack = 0;
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingEnemyWithDebuff {
                            number_of_debuffs_enemy_has,
                        } => stack += number_of_debuffs_enemy_has,
                        BattleConditionEnum::WhenAttackingEnemyWithDot {
                            number_of_dots_enemy_has,
                        } => stack += number_of_dots_enemy_has,
                        _ => (),
                    }
                }
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
                        * std::cmp::min(stack, 3) as f64;
            }
            _ => todo!(),
        }
        Ok(bonus)
    }
}
