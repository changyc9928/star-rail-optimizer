use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::{DamageType, SkillType, Stats};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CarveTheMoonWeaveTheCloudEffect {
    #[default]
    Atk,
    CritDmg,
    EnergyRegen,
}

#[derive(Clone, Debug)]
pub struct LightConeEntity {
    pub base_hp: f64,
    pub base_atk: f64,
    pub base_def: f64,
    pub _light_cone: LightCone,
    pub config: LightConePassiveConfig,
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

#[derive(Clone, Debug, Default)]
pub struct LightConePassiveConfig {
    pub activate_20000: bool,
    pub activate_20003: bool,
    pub activate_20004: bool,
    pub activate_20007: bool,
    pub activate_20009: bool,
    pub activate_20011: bool,
    pub activate_20014: bool,
    pub activate_20016: bool,
    pub activate_20019: bool,
    pub activate_20020: bool,
    pub stack_20022: u8,
    pub stack_21001: u8,
    pub activate_21003: bool,
    pub activate_21006: bool,
    pub stack_21010: u8,
    pub activate_21012: bool,
    pub activate_21015: bool,
    pub activate_21017: bool,
    pub activate_21019: bool,
    pub activate_21020: bool,
    pub activate_21022: bool,
    pub activate_21024: bool,
    pub activate_21026: bool,
    pub stack_21027: u8,
    pub activate_21032: CarveTheMoonWeaveTheCloudEffect,
    pub stack_21037: u8,
    pub activate_21038: bool,
    pub activate_21040: bool,
    pub stack_21041: u8,
    pub activate_21042: bool,
    pub stack_21043: u8,
    pub activate_21044: bool,
    pub activate_21045: bool,
    pub activate_21046: bool,
    pub activate_21047: bool,
    pub activate_21051: bool,
    pub activate_21052: bool,
    pub activate_22001: bool,
    pub activate_22002: bool,
    pub activate_22003: bool,
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
        skill_type: &SkillType,
        damage_type: &DamageType,
        base_stats: &HashMap<Stats, f64>,
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        match self._light_cone.id.as_str() {
            "20000" => {
                if self.config.activate_20000 {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20001" => {
                if *skill_type == SkillType::Skill || *skill_type == SkillType::Ultimate {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20002" => {
                if *skill_type == SkillType::BasicAttack || *skill_type == SkillType::Skill {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20003" => {
                if self.config.activate_20003 {
                    *bonus.entry(Stats::Def_).or_default() += [0.0, 16.0, 20.0, 24.0, 28.0, 32.0]
                        [self._light_cone.superimposition as usize];
                }
            }
            "20004" => {
                if self.config.activate_20004 {
                    *bonus.entry(Stats::EffectHitRate_).or_default() +=
                        [0.0, 20.0, 25.0, 30.0, 35.0, 40.0]
                            [self._light_cone.superimposition as usize];
                }
            }
            "20005" => {}
            "20006" => {
                if *skill_type == SkillType::Ultimate {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 28.0, 35.0, 42.0, 49.0, 56.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20007" => {
                if self.config.activate_20007 {
                    *bonus.entry(Stats::Atk_).or_default() += [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                        [self._light_cone.superimposition as usize]
                }
            }
            "20008" => {}
            "20009" => {
                if self.config.activate_20009 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 20.0, 25.0, 30.0, 35.0, 40.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20010" => {}
            "20011" => {
                if self.config.activate_20011 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20012" => {}
            "20013" => {}
            "20014" => {
                if self.config.activate_20014 {
                    *bonus.entry(Stats::Spd_).or_default() += [0.0, 10.0, 12.0, 14.0, 16.0, 18.0]
                        [self._light_cone.superimposition as usize]
                }
            }
            "20015" => {}
            "20016" => {
                if self.config.activate_20016 {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 12.0, 15.0, 18.0, 21.0, 24.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "20017" => {}
            "20018" => {}
            "20019" => {
                if self.config.activate_20019 {
                    *bonus.entry(Stats::Spd).or_default() += [0.0, 12.0, 14.0, 16.0, 18.0, 20.0]
                        [self._light_cone.superimposition as usize]
                }
            }
            "20020" => {
                if *skill_type == SkillType::Ultimate {
                    *bonus.entry(Stats::Atk_).or_default() += [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                        [self._light_cone.superimposition as usize]
                } else {
                    if self.config.activate_20020 {
                        *bonus.entry(Stats::Atk_).or_default() +=
                            [0.0, 24.0, 30.0, 36.0, 42.0, 48.0]
                                [self._light_cone.superimposition as usize]
                    }
                }
            }
            "20021" => {}
            "20022" => {
                *bonus.entry(Stats::DmgBoost_).or_default() += [0.0, 8.0, 9.0, 10.0, 11.0, 12.0]
                    [self._light_cone.superimposition as usize]
                    * std::cmp::min(self.config.stack_20022, 4) as f64
            }
            "21000" => {
                if *skill_type == SkillType::Ultimate {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21001" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
                        * std::cmp::min(self.config.stack_21001, 3) as f64
            }
            "21002" => {
                // TODO: After entering battle, increases All-Type RES of all allies by 8.00/9.00/10.00/11.00/12.00%. Abilities of the same type cannot stack.
            }
            "21003" => {
                if self.config.activate_21003 {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21004" => {}
            "21005" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                    [self._light_cone.superimposition as usize]
            }
            "21006" => {
                if *skill_type == SkillType::FollowUpAttack {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
                if self.config.activate_21006 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21007" => {}
            "21008" => {
                if *damage_type == DamageType::DamageOnTime {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21009" => {}
            "21010" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                        [self._light_cone.superimposition as usize]
                        * std::cmp::min(5, self.config.stack_21010) as f64;
            }
            "21011" => {
                *bonus.entry(Stats::DmgBoost_).or_default() +=
                    [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                        [self._light_cone.superimposition as usize]
            }
            "21012" => {
                if self.config.activate_21012 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21013" => {
                if *skill_type == SkillType::Ultimate {
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
                if self.config.activate_21015 {
                    *bonus.entry(Stats::DefReduction_).or_default() +=
                        [0.0, 12.00, 13.00, 14.00, 15.00, 16.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21016" => {}
            "21017" => {
                if *skill_type == SkillType::BasicAttack || *skill_type == SkillType::Skill {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize];
                    if self.config.activate_21017 {
                        *bonus.entry(Stats::DmgBoost_).or_default() +=
                            [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                                [self._light_cone.superimposition as usize];
                    }
                }
            }
            "21018" => {}
            "21019" => {
                if self.config.activate_21019 {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21020" => {
                if self.config.activate_21020 {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21021" => {}
            "21022" => {
                if self.config.activate_21022 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21023" => {}
            "21024" => {
                if self.config.activate_21024 {
                    *bonus.entry(Stats::Spd_).or_default() +=
                        [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                            [self._light_cone.superimposition as usize];
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 12.00, 15.00, 18.00, 21.00, 24.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21025" => {}
            "21026" => {
                if self.config.activate_21026 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "21027" => {
                *bonus.entry(Stats::Atk_).or_default() += [0.0, 4.00, 5.00, 6.00, 7.00, 8.00]
                    [self._light_cone.superimposition as usize]
                    * std::cmp::min(3, self.config.stack_21027) as f64;
            }
            "21028" => {}
            "21029" => {}
            "21030" => {
                if *skill_type == SkillType::Ultimate {
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
            "21032" => match self.config.activate_21032 {
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
            },
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
                *bonus.entry(Stats::CritDmg_).or_default() += [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                    [self._light_cone.superimposition as usize]
                    * std::cmp::min(self.config.stack_21037, 4) as f64
            }
            "21038" => {
                if self.config.activate_21038 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 25.00, 31.25, 37.50, 43.75, 50.00]
                            [self._light_cone.superimposition as usize]
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
                if self.config.activate_21040 {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21041" => {
                *bonus.entry(Stats::DmgBoost_).or_default() += [0.0, 6.00, 7.00, 8.00, 9.00, 10.00]
                    [self._light_cone.superimposition as usize]
                    * std::cmp::min(self.config.stack_21041, 3) as f64;

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
                if *skill_type == SkillType::Ultimate || self.config.activate_21042 {
                    *bonus.entry(Stats::CritRate_).or_default() +=
                        [0.0, 15.00, 18.75, 22.50, 26.25, 30.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21043" => {
                *bonus.entry(Stats::CritRate_).or_default() += [0.0, 4.00, 5.00, 6.00, 7.00, 8.00]
                    [self._light_cone.superimposition as usize]
                    * self.config.stack_21043 as f64
            }
            "21044" => {
                if self.config.activate_21044 {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 24.00, 30.00, 36.00, 42.00, 48.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21045" => {
                if self.config.activate_21045 {
                    *bonus.entry(Stats::Spd_).or_default() +=
                        [0.0, 8.00, 10.00, 12.00, 14.00, 16.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21046" => {
                if self.config.activate_21046 {
                    *bonus.entry(Stats::CritDmg_).or_default() +=
                        [0.0, 16.00, 20.00, 24.00, 28.00, 32.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21047" => {
                if self.config.activate_21047 {
                    *bonus.entry(Stats::Spd_).or_default() += [0.0, 8.00, 9.00, 10.00, 11.00, 12.00]
                        [self._light_cone.superimposition as usize]
                }
            }
            "21048" => {}
            "21050" => {}
            "21051" => {
                if self.config.activate_21051 && *skill_type == SkillType::BasicAttack {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 20.00, 25.00, 30.00, 35.00, 40.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "21052" => {
                if self.config.activate_21052 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 24.00, 27.00, 30.00, 33.00, 36.00]
                            [self._light_cone.superimposition as usize]
                }
            }
            "22000" => {}
            "22001" => {
                if self.config.activate_22001 {
                    *bonus.entry(Stats::OutgoingHealingBoost_).or_default() +=
                        [0.0, 16.0, 19.0, 22.0, 25.0, 28.0]
                            [self._light_cone.superimposition as usize]
                }
            }
            "22002" => {
                if self.config.activate_22002 {
                    *bonus.entry(Stats::DmgBoost_).or_default() +=
                        [0.0, 18.00, 21.00, 24.00, 27.00, 30.00]
                            [self._light_cone.superimposition as usize];
                }
            }
            "22003" => {
                if self.config.activate_22003 {
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
