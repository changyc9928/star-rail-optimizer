use eyre::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BattleConditionEnum, CarveTheMoonWeaveTheCloudEffect, Stats, Tag};

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
    pub fn get_bonus_before_battle(&self) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        match self._light_cone.id.as_str() {
            "20000" => {}
            "20001" => {}
            "20002" => {}
            "20003" => {
                *bonus.entry(Stats::Def_).or_default() +=
                    [0.0, 16.0, 20.0, 24.0, 28.0, 32.0][self._light_cone.superimposition as usize];
            }
            "20004" => {}
            "20005" => {}
            "20006" => {}
            "20007" => {}
            "20008" => {}
            "20009" => {}
            "20010" => {}
            "20011" => {}
            "20012" => {}
            "20013" => {}
            "20014" => {}
            "20015" => {}
            "20016" => {}
            "20017" => {}
            "20018" => {}
            "20019" => {}
            "20020" => {}
            "20021" => {}
            "20022" => {}
            "21000" => {
                *bonus.entry(Stats::EnergyRegenerationRate_).or_default() +=
                    [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                        [self._light_cone.superimposition as usize]
            }
            "21001" => {}
            "21002" => {
                *bonus.entry(Stats::Def_).or_default() += [0.0, 16.00, 18.00, 20.00, 22.00, 24.00]
                    [self._light_cone.superimposition as usize]
            }
            "21003" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21004" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 28.00, 35.00, 42.00, 49.00, 56.00]
                        [self._light_cone.superimposition as usize]
            }
            "21005" => {}
            "21006" => {}
            "21007" => {
                *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                    [0.0, 10.00, 12.50, 15.00, 17.50, 20.00]
                        [self._light_cone.superimposition as usize]
            }
            "21008" => {
                *bonus.entry(Stats::EffectHitRate_).or_default() +=
                    [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                        [self._light_cone.superimposition as usize]
            }
            "21009" => {
                *bonus.entry(Stats::DmgMitigation_).or_default() +=
                    [0.0, 16.00, 18.00, 20.00, 22.00, 24.00]
                        [self._light_cone.superimposition as usize]
            }
            "21010" => {}
            "21011" => {}
            "21012" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                        [self._light_cone.superimposition as usize]
            }
            "21013" => {}
            "21014" => {
                *bonus.entry(Stats::EffectRes_).or_default() +=
                    [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                        [self._light_cone.superimposition as usize]
            }
            "21015" => {}
            "21016" => {
                *bonus.entry(Stats::Def_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21017" => {}
            "21018" => {}
            "21019" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21020" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21021" => {}
            "21022" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                        [self._light_cone.superimposition as usize]
            }
            "21023" => {}
            "21024" => {}
            "21025" => {}
            "21026" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 10.00, 12.50, 15.00, 17.50, 20.00]
                    [self._light_cone.superimposition as usize]
            }
            "21027" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
            }
            "21028" => {
                *bonus.entry(Stats::Hp_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21029" => {}
            "21030" => {
                *bonus.entry(Stats::Def_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21031" => {
                *bonus.entry(Stats::CritRate_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
            }
            "21032" => {}
            "21033" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                    [self._light_cone.superimposition as usize]
            }
            "21034" => {}
            "21035" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                        [self._light_cone.superimposition as usize]
            }
            "21036" => {}
            "21037" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 12.00, 14.00, 16.00, 18.00, 20.00]
                    [self._light_cone.superimposition as usize]
            }
            "21038" => {}
            "21039" => {
                *bonus.entry(Stats::EffectRes_).or_default() +=
                    [0.0, 12.00, 14.00, 16.00, 18.00, 20.00]
                        [self._light_cone.superimposition as usize]
            }
            "21040" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 18.00, 20.00, 22.00, 24.00]
                    [self._light_cone.superimposition as usize]
            }
            "21041" => {}
            "21042" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 28.00, 35.00, 42.00, 49.00, 56.00]
                        [self._light_cone.superimposition as usize]
            }
            "21043" => {
                *bonus.entry(Stats::Def_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize]
            }
            "21044" => {
                *bonus.entry(Stats::CritRate_).or_default() +=
                    [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                        [self._light_cone.superimposition as usize]
            }
            "21045" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 28.00, 35.00, 42.00, 49.00, 56.00]
                        [self._light_cone.superimposition as usize];
            }
            "21046" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize];
            }
            "21047" => {
                *bonus.entry(Stats::BreakEffect_).or_default() +=
                    [0.0, 28.00, 35.00, 42.00, 49.00, 56.00]
                        [self._light_cone.superimposition as usize];
            }
            "21048" => {
                *bonus.entry(Stats::Spd_).or_default() += [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                    [self._light_cone.superimposition as usize];
            }
            "21050" => {
                *bonus.entry(Stats::CritDmg_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize];
            }
            "21051" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                    [self._light_cone.superimposition as usize];
            }
            "21052" => {
                *bonus.entry(Stats::CritRate_).or_default() +=
                    [0.0, 12.00, 14.00, 16.00, 18.00, 20.00]
                        [self._light_cone.superimposition as usize];
            }
            "22000" => {
                *bonus.entry(Stats::EffectHitRate_).or_default() +=
                    [0.0, 20.0, 25.0, 30.0, 35.0, 40.0][self._light_cone.superimposition as usize]
            }
            "22001" => {
                *bonus.entry(Stats::Hp_).or_default() +=
                    [0.0, 8.0, 9.0, 10.0, 11.0, 12.0][self._light_cone.superimposition as usize];
            }
            "22002" => {
                *bonus.entry(Stats::Atk_).or_default() +=
                    [0.0, 16.0, 20.0, 24.0, 28.0, 32.0][self._light_cone.superimposition as usize];
            }
            "22003" => {
                *bonus.entry(Stats::Hp_).or_default() += [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                    [self._light_cone.superimposition as usize];
            }
            _ => todo!(),
        }
        Ok(bonus)
    }

    pub fn get_bonus_during_battle(
        &self,
        tags: &[Tag],
        base_stats: &HashMap<Stats, f64>,
        battle_conditions: &[BattleConditionEnum],
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        match self._light_cone.id.as_str() {
            "20000" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfTurnsSinceStartOfBattle {
                            number_of_turns,
                        } => {
                            if *number_of_turns <= 3 {
                                *bonus.entry(Stats::CritRate_).or_default() +=
                                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            "20001" => {
                if tags.contains(&Tag::Skill) || tags.contains(&Tag::Ultimate) {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20002" => {
                if tags.contains(&Tag::BasicAtk) || tags.contains(&Tag::Skill) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20003" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WearerCurrentHpPercentage { percentage } => {
                            if *percentage < 50.0 {
                                *bonus.entry(Stats::Def_).or_default() +=
                                    [0.0, 16.0, 20.0, 24.0, 28.0, 32.0]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            "20004" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfTurnsSinceStartOfBattle {
                            number_of_turns,
                        } => {
                            if *number_of_turns <= 3 {
                                *bonus.entry(Stats::EffectHitRate_).or_default() +=
                                    [0.0, 20.0, 25.0, 30.0, 35.0, 40.0]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            "20005" => {}
            "20006" => {
                if tags.contains(&Tag::Ultimate) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 28.0, 35.0, 42.0, 49.0, 56.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20007" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterEnemyDefeated {
                            number_of_turns_after_enemy_defeated,
                            ..
                        } => {
                            if *number_of_turns_after_enemy_defeated <= 3 {
                                *bonus.entry(Stats::Atk_).or_default() +=
                                    [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "20008" => {}
            "20009" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::TargetEnemyCurrentHpPercentage { percentage } => {
                            if *percentage > 50.0 {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 20.0, 25.0, 30.0, 35.0, 40.0]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "20010" => {}
            "20011" => {
                if battle_conditions.contains(&BattleConditionEnum::WhenAttackingSlowedDownEnemy) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20012" => {}
            "20013" => {}
            "20014" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterEnemyDefeated { .. } => {
                            *bonus.entry(Stats::Spd_).or_default() +=
                                [0.0, 10.0, 12.0, 14.0, 16.0, 18.0]
                                    [self._light_cone.superimposition as usize]
                        }
                        _ => (),
                    }
                }
            }
            "20015" => {}
            "20016" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WearerCurrentHpPercentage { percentage } => {
                            if *percentage < 80.0 {
                                *bonus.entry(Stats::CritRate_).or_default() +=
                                    [0.0, 12.0, 15.0, 18.0, 21.0, 24.0]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "20017" => {}
            "20018" => {}
            "20019" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfTurnsSinceStartOfBattle {
                            number_of_turns,
                        } => {
                            if *number_of_turns <= 1 {
                                *bonus.entry(Stats::Spd).or_default() +=
                                    [0.0, 12.0, 14.0, 16.0, 18.0, 20.0]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "20020" => {
                if tags.contains(&Tag::Ultimate) {
                    *bonus.entry(Stats::Atk_).or_default() += [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                        [self._light_cone.superimposition as usize]
                } else {
                    for condition in battle_conditions {
                        match condition {
                            BattleConditionEnum::AfterUsingUltimate {
                                number_of_turns_since_using_ultimate,
                                ..
                            } => {
                                if *number_of_turns_since_using_ultimate <= 2 {
                                    *bonus.entry(Stats::Atk_).or_default() +=
                                        [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                                            [self._light_cone.superimposition as usize]
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            "20021" => {}
            "20022" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::MemospriteTurnSinceSummoned { number_of_turns } => {
                            *bonus.entry(Stats::DmgBoost_).or_default() +=
                                [0.0, 8.0, 9.0, 10.0, 11.0, 12.0]
                                    [self._light_cone.superimposition as usize]
                                    * std::cmp::min(*number_of_turns, 4) as f64
                        }
                        _ => (),
                    }
                }
            }
            "21000" => {
                if tags.contains(&Tag::Ultimate) {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21001" => {
                let mut stack = 0;
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingEnemyWithDebuff {
                            number_of_debuffs_enemy_has,
                            ..
                        } => stack += number_of_debuffs_enemy_has,
                        BattleConditionEnum::WhenAttackingEnemyWithDot {
                            number_of_dots_enemy_has,
                            ..
                        } => stack += number_of_dots_enemy_has,
                        _ => (),
                    }
                }
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
                        * std::cmp::min(stack, 3) as f64;
            }
            "21002" => {
                // TODO: After entering battle, increases All-Type RES of all allies by 8.00/9.00/10.00/11.00/12.00%. Abilities of the same type cannot stack.
            }
            "21003" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfEnemiesOnField(number) => {
                            if *number <= 2 {
                                *bonus.entry(Stats::CritRate_).or_default() +=
                                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "21004" => {}
            "21005" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                    [self._light_cone.superimposition as usize]
            }
            "21006" => {
                if tags.contains(&Tag::FollowUpAtk) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::TargetEnemyCurrentHpPercentage { percentage } => {
                            if *percentage <= 50.0 {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "21007" => {}
            "21008" => {
                if tags.contains(&Tag::Dot) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21009" => {}
            "21010" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AttackingSameEnemy { number_of_times } => {
                            *bonus.entry(Stats::DmgBoost_).or_default() +=
                                [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                                    [self._light_cone.superimposition as usize]
                                    * std::cmp::min(5, *number_of_times) as f64;
                        }
                        _ => {}
                    }
                }
            }
            "21011" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
            }
            "21012" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::EnemyHpPercentHigherThanWearerHpPercent(flag) => {
                            if *flag {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21013" => {
                if tags.contains(&Tag::Ultimate) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 32.00, 40.00, 48.00, 56.00, 64.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21014" => {
                let b = [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                    [self._light_cone.superimposition as usize]
                    * base_stats
                        .get(&Stats::EffectRes_)
                        .ok_or_else(|| eyre::eyre!("Missing effect res from the character"))?;
                let cap = [0.0, 15.00, 18.00, 21.00, 24.00, 27.00]
                    [self._light_cone.superimposition as usize];
                if b > cap {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() += cap;
                } else {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() += b;
                }
            }
            "21015" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::EnemyEnsnared {
                            within_number_of_turns,
                        } => {
                            if *within_number_of_turns <= 1 {
                                *bonus.entry(Stats::DefReduction_).or_default() +=
                                    [0.0, 12.00, 13.00, 14.00, 15.00, 16.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "21016" => {}
            "21017" => {
                if tags.contains(&Tag::BasicAtk) || tags.contains(&Tag::Skill) {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize];
                    if battle_conditions.contains(&BattleConditionEnum::WearerEnergyReachesMax) {
                        *bonus.entry(Stats::DmgBoost_).or_default() +=
                            [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                                [self._light_cone.superimposition as usize];
                    }
                }
            }
            "21018" => {}
            "21019" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterEnemyDefeated {
                            number_of_turns_after_enemy_defeated,
                            ..
                        } => {
                            if *number_of_turns_after_enemy_defeated <= 3 {
                                *bonus.entry(Stats::CritRate_).or_default() +=
                                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21020" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterEnemyDefeated {
                            number_of_turns_after_enemy_defeated,
                            ..
                        } => {
                            if *number_of_turns_after_enemy_defeated <= 3 {
                                *bonus.entry(Stats::CritDmg_).or_default() +=
                                    [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21021" => {}
            "21022" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingEnemyWithDot { debuffs, .. } => {
                            for debuff in debuffs {
                                if debuff.debuff == "Shock" || debuff.debuff == "Wind Shear" {
                                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                                        [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                                            [self._light_cone.superimposition as usize];
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21023" => {}
            "21024" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterWearerIsHit {
                            within_number_of_turns,
                            ..
                        } => {
                            if *within_number_of_turns >= 1 {
                                *bonus.entry(Stats::Spd_).or_default() +=
                                    [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                                        [self._light_cone.superimposition as usize];
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => {}
                    }
                }
            }
            "21025" => {}
            "21026" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingEnemyWithDot { debuffs, .. } => {
                            for debuff in debuffs {
                                if debuff.debuff == "Burn" || debuff.debuff == "Bleed" {
                                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                                        [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                                            [self._light_cone.superimposition as usize];
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21027" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterEnemyDefeated {
                            number_of_enemies_defeated,
                            ..
                        } => {
                            *bonus.entry(Stats::Atk_).or_default() +=
                                [0.0, 4.00, 5.00, 6.00, 7.00, 8.00]
                                    [self._light_cone.superimposition as usize]
                                    * std::cmp::min(3, *number_of_enemies_defeated) as f64;
                        }
                        _ => (),
                    }
                }
            }
            "21028" => {}
            "21029" => {}
            "21030" => {
                if tags.contains(&Tag::Ultimate) {
                    let def = base_stats
                        .get(&Stats::Def)
                        .ok_or_else(|| eyre::eyre!("Missing character's DEF"))?;
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 60.00, 75.00, 90.00, 105.00, 120.00]
                            [self._light_cone.superimposition as usize]
                            * def;
                }
            }
            "21031" => {}
            "21032" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::CarveTheMoonWeaveTheCloudEffect { effect } => {
                            match effect {
                                CarveTheMoonWeaveTheCloudEffect::Atk => {
                                    *bonus.entry(Stats::Atk_).or_default() +=
                                        [0.0, 10.00, 12.50, 15.00, 17.50, 20.00]
                                            [self._light_cone.superimposition as usize]
                                }
                                CarveTheMoonWeaveTheCloudEffect::CritDmg => {
                                    *bonus.entry(Stats::CritDmg_).or_default() +=
                                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                                            [self._light_cone.superimposition as usize]
                                }
                                CarveTheMoonWeaveTheCloudEffect::EnergyRegen => {
                                    *bonus.entry(Stats::EnergyRegenerationRate_).or_default() +=
                                        [0.0, 6.00, 7.50, 9.00, 10.50, 12.00]
                                            [self._light_cone.superimposition as usize]
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21033" => {}
            "21034" => {
                let val = [0.0, 0.20, 0.25, 0.30, 0.35, 0.40]
                    [self._light_cone.superimposition as usize]
                    * base_stats
                        .get(&Stats::EnergyRegenerationRate_)
                        .ok_or_else(|| {
                            eyre::eyre!("Missing character's energy regeneration rate")
                        })?;
                if val < 160.0 {
                    *bonus.entry(Stats::DmgBoost_).or_default() += val;
                } else {
                    *bonus.entry(Stats::DmgBoost_).or_default() += 160.0;
                }
            }
            "21035" => {}
            "21036" => {}
            "21037" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfCriticalAttackDealtToEnemiesBeforeTurnEnds {
                            number_of_crits
                        } => {
                            *bonus.entry(Stats::CritDmg_).or_default() +=
                                [0.0, 8.00,9.00,10.00,11.00,12.00]
                                    [self._light_cone.superimposition as usize] * std::cmp::min(*number_of_crits, 4) as f64
                        }
                        _ => (),
                    }
                }
            }
            "21038" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenWearerLosingHp {
                            within_number_of_turns,
                            percentage_of_hp_lost_in_one_turn,
                            ..
                        } => {
                            if *within_number_of_turns <= 2
                                && *percentage_of_hp_lost_in_one_turn <= 25.0
                            {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 25.00, 31.25, 37.50, 43.75, 50.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21039" => {
                let addition = [0.0, 0.80, 0.90, 1.00, 1.10, 1.20]
                    [self._light_cone.superimposition as usize]
                    * (base_stats.get(&Stats::Def).cloned().unwrap_or_default() / 100.0);
                let cap = [0.0, 32.00, 36.00, 40.00, 44.00, 48.00]
                    [self._light_cone.superimposition as usize];
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    if addition <= cap { addition } else { cap }
            }
            "21040" => {
                let mut flag = false;
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingEnemyWithCorrespondingWeakness {
                            number_of_enemies,
                            within_number_of_turns,
                        } => {
                            if *number_of_enemies >= 2 && *within_number_of_turns <= 2 {
                                flag = true
                            }
                        }
                        _ => (),
                    }
                }
                if flag {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21041" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterWearerInflictingDebuffs {
                            number_of_times,
                            within_number_of_turns,
                        } => {
                            if *within_number_of_turns <= 1 {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 6.00, 7.00, 8.00, 9.00, 10.00]
                                        [self._light_cone.superimposition as usize]
                                        * std::cmp::min(*number_of_times, 3) as f64
                            }
                        }
                        _ => {}
                    }
                }
                if *base_stats
                    .get(&Stats::EffectHitRate_)
                    .ok_or_else(|| eyre::eyre!("Missing character's effect hit rate"))?
                    >= 80.0
                {
                    *bonus.entry(Stats::Atk_).or_default() +=
                        [0.0, 20.00, 24.00, 28.00, 32.00, 36.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21042" => {
                let mut flag = false;
                if tags.contains(&Tag::Ultimate) {
                    flag = true;
                } else {
                    for condition in battle_conditions {
                        match condition {
                            BattleConditionEnum::AfterUsingUltimate {
                                number_of_turns_since_using_ultimate,
                                ..
                            } => {
                                if *number_of_turns_since_using_ultimate <= 2 {
                                    flag = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if flag {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 15.00, 18.75, 22.50, 26.25, 30.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21043" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfCharactersHavingShield {
                            number_of_characters,
                        } => {
                            *bonus.entry(Stats::CritRate_).or_default() +=
                                [0.0, 4.00, 5.00, 6.00, 7.00, 8.00]
                                    [self._light_cone.superimposition as usize]
                                    * *number_of_characters as f64
                        }
                        _ => {}
                    }
                }
            }
            "21044" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenAttackingSlowedDownEnemy
                        | BattleConditionEnum::WhenAttackingReducedDefEnemy => {
                            *bonus.entry(Stats::CritDmg_).or_default() +=
                                [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                                    [self._light_cone.superimposition as usize]
                        }
                        _ => (),
                    }
                }
            }
            "21045" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterUsingUltimate {
                            number_of_turns_since_using_ultimate,
                            ..
                        } => {
                            if *number_of_turns_since_using_ultimate <= 2 {
                                *bonus.entry(Stats::Spd_).or_default() +=
                                    [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21046" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::TeammatesSamePathWithWearer {
                            number_of_teammates_having_same_path,
                        } => {
                            if *number_of_teammates_having_same_path >= 2 {
                                *bonus.entry(Stats::CritDmg_).or_default() +=
                                    [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21047" => {
                let mut flag = false;
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::NumberOfTurnsSinceStartOfBattle {
                            number_of_turns,
                        } => {
                            if *number_of_turns <= 2 {
                                flag = true;
                            }
                        }
                        BattleConditionEnum::AfterDealingBreakDmg {
                            within_number_of_turns,
                        } => {
                            if *within_number_of_turns <= 2 {
                                flag = true
                            }
                        }
                        _ => (),
                    }
                }
                if flag {
                    *bonus.entry(Stats::Spd_).or_default() += [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                        [self._light_cone.superimposition as usize]
                }
            }
            "21048" => {}
            "21050" => {}
            "21051" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterUsingUltimate {
                            number_of_turns_since_using_ultimate,
                            ..
                        } => {
                            if *number_of_turns_since_using_ultimate <= 3
                                && tags.contains(&Tag::BasicAtk)
                            {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "21052" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenMemospriteIsOnField { on_field } => {
                            if *on_field {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 24.00, 27.00, 30.00, 33.00, 36.00]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => {}
                    }
                }
            }
            "22000" => {}
            "22001" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterUsingSkill {
                            number_of_turns_since_using_the_skill,
                        } => {
                            if *number_of_turns_since_using_the_skill <= 2 {
                                *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                                    [0.0, 16.0, 19.0, 22.0, 25.0, 28.0]
                                        [self._light_cone.superimposition as usize]
                            }
                        }
                        _ => (),
                    }
                }
            }
            "22002" => {
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::AfterUsingUltimate {
                            number_of_turns_since_using_ultimate,
                            ..
                        } => {
                            if *number_of_turns_since_using_ultimate <= 1 {
                                *bonus.entry(Stats::DmgBoost_).or_default() +=
                                    [0.0, 18.00, 21.00, 24.00, 27.00, 30.00]
                                        [self._light_cone.superimposition as usize];
                            }
                        }
                        _ => {}
                    }
                }
            }
            "22003" => {
                let mut flag = false;
                for condition in battle_conditions {
                    match condition {
                        BattleConditionEnum::WhenWearerLosingHp {
                            within_number_of_turns,
                            ..
                        } => {
                            if *within_number_of_turns <= 2 {
                                flag = true
                            }
                        }
                        BattleConditionEnum::WhenWearerGainingHp {
                            within_number_of_turns,
                        } => {
                            if *within_number_of_turns <= 2 {
                                flag = true
                            }
                        }
                        _ => (),
                    }
                }
                if flag {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 18.00, 22.50, 27.00, 31.50, 36.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            _ => todo!(),
        }
        Ok(bonus)
    }
}
