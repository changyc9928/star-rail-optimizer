use eyre::{eyre, Result};
use itertools::Itertools;
use std::{cmp::min, collections::HashMap};

use crate::{
    character::Support,
    domain::{AttackType, DamageType, Path, SkillType},
};

use super::{Relic, Stats};

#[derive(Clone, Debug)]
pub struct Relics {
    pub relics: Vec<Relic>,
    pub config: RelicSetConfig,
}

#[derive(Clone, Debug)]
pub struct RelicSetConfig {
    pub activate_102: bool,
    pub activate_104: bool,
    pub stack_105: u8,
    pub activate_107: bool,
    pub activate_108: bool,
    pub activate_109: bool,
    pub activate_112_1: bool,
    pub activate_112_2: bool,
    pub stack_113: u8,
    pub stack_115: u8,
    pub stack_116: u8,
    pub activate_117_2pcs: bool,
    pub stack_117: u8,
    pub activate_117_4pcs_extra: bool,
    pub activate_120: bool,
    pub activate_122: bool,
    pub activate_123_1: bool,
    pub activate_123_2: bool,
    pub activate_125: bool,
    pub activate_126: bool,
    pub activate_305: bool,
    pub stack_313: u8,
    pub stack_315: u8,
    pub activate_316: bool,
    pub activate_318: bool,
    pub stack_321: u8,
}

impl Relics {
    pub fn calculate_bonus_before_battle(
        &self,
        attack_type: &AttackType,
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        self.calculate_bonus(attack_type, &mut bonus)?;
        self.calculate_set_bonus(attack_type, &mut bonus)?;
        Ok(bonus)
    }

    pub fn calculate_bonus(
        &self,
        attack_type: &AttackType,
        bonus: &mut HashMap<Stats, f64>,
    ) -> Result<()> {
        for relic in &self.relics {
            for substat in &relic.substats {
                *bonus.entry(substat.key.clone()).or_default() += substat.value;
            }
            if *attack_type == AttackType::Lightning && relic.mainstat == Stats::LightningDmgBoost_
            {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Wind && relic.mainstat == Stats::WindDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Fire && relic.mainstat == Stats::FireDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Ice && relic.mainstat == Stats::IceDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Quantum
                && relic.mainstat == Stats::QuantumDmgBoost_
            {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Imaginary
                && relic.mainstat == Stats::ImaginaryDmgBoost_
            {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if *attack_type == AttackType::Physical
                && relic.mainstat == Stats::PhysicalDmgBoost_
            {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else {
                *bonus.entry(relic.mainstat.clone()).or_default() += relic.get_mainstat()?;
            }
        }
        Ok(())
    }

    pub fn calculate_set_bonus(
        &self,
        attack_type: &AttackType,
        bonus: &mut HashMap<Stats, f64>,
    ) -> Result<()> {
        let count = self.relics.iter().counts_by(|r| r.set_id.clone());
        for (set_id, num_relics) in count {
            match set_id.as_str() {
                "101" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::OutgoingHealingBoost_).or_default() += 10.0;
                    }
                }
                "102" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "103" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Def_).or_default() += 15.0;
                    }
                }
                "104" => {
                    if num_relics >= 2 && *attack_type == AttackType::Ice {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "105" => {
                    if num_relics >= 2 && *attack_type == AttackType::Physical {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "106" => {}
                "107" => {
                    if num_relics >= 2 && *attack_type == AttackType::Fire {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "108" => {
                    if num_relics >= 2 && *attack_type == AttackType::Quantum {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                    }
                }
                "109" => {
                    if num_relics >= 2 && *attack_type == AttackType::Lightning {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "110" => {
                    if num_relics >= 2 && *attack_type == AttackType::Wind {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "111" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 16.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 16.0;
                    }
                }
                "112" => {
                    if num_relics >= 2 && *attack_type == AttackType::Imaginary {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "113" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Hp_).or_default() += 12.0;
                    }
                }
                "114" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "115" => {}
                "116" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                "117" => {
                    if num_relics >= 4 {
                        *bonus.entry(Stats::CritRate_).or_default() += 4.0;
                    }
                }
                "118" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 16.0;
                    }
                }
                "119" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 16.0;
                    }
                }
                "120" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::CritRate_).or_default() += 6.0;
                    }
                }
                "121" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "122" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritRate_).or_default() += 8.0;
                    }
                }
                "123" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                "124" => {
                    if num_relics >= 2 && *attack_type == AttackType::Quantum {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::Spd_).or_default() -= 8.0;
                    }
                }
                "125" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "126" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritDmg_).or_default() += 16.0;
                    }
                }
                "301" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                "302" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Hp_).or_default() += 12.0;
                    }
                }
                "303" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::EffectHitRate_).or_default() += 10.0;
                    }
                }
                "304" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Def_).or_default() += 15.0;
                    }
                }
                "305" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritDmg_).or_default() += 16.0;
                    }
                }
                "306" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritRate_).or_default() += 8.0;
                    }
                }
                "307" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 16.0;
                    }
                }
                "308" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::EnergyRegenerationRate_).or_default() += 5.0;
                    }
                }
                "309" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritRate_).or_default() += 8.0;
                    }
                }
                "310" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::EffectRes_).or_default() += 10.0;
                    }
                }
                "311" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                "312" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::EnergyRegenerationRate_).or_default() += 5.0;
                    }
                }
                "313" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritRate_).or_default() += 4.0;
                    }
                }
                "314" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                "315" => {}
                "316" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "317" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::EnergyRegenerationRate_).or_default() += 5.0;
                    }
                }
                "318" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritDmg_).or_default() += 16.0;
                    }
                }
                "319" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Hp_).or_default() += 12.0;
                    }
                }
                "320" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                    }
                }
                "321" => {}
                "322" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::Atk_).or_default() += 12.0;
                    }
                }
                _ => todo!(),
            }
        }
        Ok(())
    }

    pub fn calculate_bonus_during_battle(
        &self,
        path: Path,
        attack_type: &AttackType,
        skill_type: &SkillType,
        damage_type: &DamageType,
        base_stats: &HashMap<Stats, f64>,
        teammates: &[Box<dyn Support>],
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        let count = self.relics.iter().counts_by(|r| r.set_id.clone());
        for (set_id, num_relics) in count {
            match set_id.as_str() {
                "101" => {}
                "102" => {
                    if num_relics >= 4
                        && self.config.activate_102
                        && *skill_type == SkillType::BasicAttack
                    {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "103" => {}
                "104" => {
                    if num_relics >= 4 && self.config.activate_104 {
                        *bonus.entry(Stats::CritDmg_).or_default() += 25.0;
                    }
                }
                "105" => {
                    if num_relics >= 4 {
                        let stack = min(5, self.config.stack_105);
                        *bonus.entry(Stats::Atk_).or_default() += 5.0 * stack as f64;
                    }
                }
                "107" => {
                    if num_relics >= 4 {
                        if *skill_type == SkillType::Skill {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                        }
                        if *attack_type == AttackType::Fire && self.config.activate_107 {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                        }
                    }
                }
                "108" => {
                    if num_relics >= 4 && self.config.activate_108 {
                        *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                    }
                }
                "109" => {
                    if num_relics >= 4 && self.config.activate_109 {
                        *bonus.entry(Stats::Atk_).or_default() += 20.0;
                    }
                }
                "110" => {}
                "111" => {}
                "112" => {
                    if num_relics >= 4 {
                        if self.config.activate_112_1 {
                            *bonus.entry(Stats::CritRate_).or_default() += 10.0;
                        }
                        if self.config.activate_112_2 {
                            *bonus.entry(Stats::CritDmg_).or_default() += 20.0;
                        }
                    }
                }
                "113" => {
                    if num_relics >= 4 {
                        let stack = min(2, self.config.stack_113);
                        *bonus.entry(Stats::CritRate_).or_default() += 8.0 * stack as f64;
                    }
                }
                "114" => {}
                "115" => {
                    if num_relics >= 2 && *skill_type == SkillType::FollowUpAttack {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                    }
                    if num_relics >= 4 {
                        let stack = min(8, self.config.stack_115);
                        *bonus.entry(Stats::Atk_).or_default() += 6.0 * stack as f64;
                    }
                }
                "116" => {
                    if num_relics >= 4 {
                        let stack = min(3, self.config.stack_116);
                        *bonus.entry(Stats::DefIgnore_).or_default() += 6.0 * stack as f64;
                    }
                }
                "117" => {
                    if num_relics >= 2 && self.config.activate_117_2pcs {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                    }
                    if num_relics >= 4 {
                        if self.config.stack_117 == 2 {
                            if self.config.activate_117_4pcs_extra {
                                *bonus.entry(Stats::CritDmg_).or_default() += 16.0;
                            } else {
                                *bonus.entry(Stats::CritDmg_).or_default() += 8.0;
                            }
                        } else if self.config.stack_117 >= 3 {
                            if self.config.activate_117_4pcs_extra {
                                *bonus.entry(Stats::CritDmg_).or_default() += 24.0;
                            } else {
                                *bonus.entry(Stats::CritDmg_).or_default() += 12.0;
                            }
                        }
                    }
                }
                "118" => {}
                "119" => {
                    if num_relics >= 4 {
                        if *base_stats
                            .get(&Stats::BreakEffect_)
                            .ok_or(eyre!("Missing break effect"))?
                            >= 150.0
                            && *damage_type == DamageType::BreakDamage
                        {
                            *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                        } else if *base_stats
                            .get(&Stats::BreakEffect_)
                            .ok_or(eyre!("Missing break effect"))?
                            >= 250.0
                            && *damage_type == DamageType::SuperBreakDamage
                        {
                            *bonus.entry(Stats::DefIgnore_).or_default() += 25.0;
                        }
                    }
                }
                "120" => {
                    if num_relics >= 4
                        && self.config.activate_120
                        && *skill_type == SkillType::Ultimate
                    {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 36.0;
                    }
                }
                "121" => {}
                "122" => {
                    if num_relics >= 4 {
                        if *skill_type == SkillType::Skill || *skill_type == SkillType::Ultimate {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                        }
                        if *skill_type == SkillType::Skill && self.config.activate_122 {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 25.0;
                        }
                    }
                }
                "123" => {
                    if num_relics >= 4 {
                        if self.config.activate_123_1 {
                            *bonus.entry(Stats::Spd_).or_default() += 6.0;
                        }
                        if self.config.activate_123_2 {
                            *bonus.entry(Stats::CritDmg_).or_default() += 30.0;
                        }
                    }
                }
                "124" => {
                    if num_relics >= 4 {
                        if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? < 95.0 {
                            *bonus.entry(Stats::CritRate_).or_default() += 32.0;
                        } else if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? < 110.0
                        {
                            *bonus.entry(Stats::CritRate_).or_default() += 20.0;
                        }
                    }
                }
                "125" => {
                    if num_relics >= 4 && self.config.activate_125 {
                        *bonus.entry(Stats::Spd_).or_default() += 6.0;
                        *bonus.entry(Stats::CritDmg_).or_default() += 15.0;
                    }
                }
                "126" => {
                    if num_relics >= 4
                        && self.config.activate_126
                        && *skill_type == SkillType::Ultimate
                    {
                        *bonus.entry(Stats::Atk_).or_default() += 48.0;
                    }
                }
                "301" => {
                    if num_relics >= 2 {
                        if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 120.0 {
                            *bonus.entry(Stats::Atk_).or_default() += 12.0;
                        }
                    }
                }
                "302" => {}
                "303" => {
                    if num_relics >= 2 {
                        let curr_effect_hit_rate = base_stats
                            .get(&Stats::EffectHitRate_)
                            .ok_or(eyre!("Missing effect hit rate"))?;
                        let atk_bonus = curr_effect_hit_rate * 0.25;
                        if atk_bonus <= 25.0 {
                            *bonus.entry(Stats::Atk_).or_default() += atk_bonus;
                        } else {
                            *bonus.entry(Stats::Atk_).or_default() += 25.0;
                        }
                    }
                }
                "304" => {
                    if num_relics >= 2 {
                        if *base_stats
                            .get(&Stats::EffectHitRate_)
                            .ok_or(eyre!("Missing effect hit rate"))?
                            >= 50.0
                        {
                            *bonus.entry(Stats::Def_).or_default() += 15.0;
                        }
                    }
                }
                "305" => {
                    if num_relics >= 2 {
                        if self.config.activate_305
                            && *base_stats
                                .get(&Stats::CritDmg_)
                                .ok_or(eyre!("Missing CRIT DMG"))?
                                >= 120.0
                        {
                            *bonus.entry(Stats::CritRate_).or_default() += 60.0;
                        }
                    }
                }
                "306" => {
                    if num_relics >= 2 {
                        if *base_stats
                            .get(&Stats::CritRate_)
                            .ok_or(eyre!("Missing CRIT Rate"))?
                            >= 50.0
                            && *skill_type == SkillType::Ultimate
                            || *skill_type == SkillType::FollowUpAttack
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 15.0;
                        }
                    }
                }
                "307" => {
                    if num_relics >= 2 {
                        if *base_stats
                            .get(&Stats::Spd)
                            .ok_or(eyre!("Missing CRIT Rate"))?
                            >= 145.0
                        {
                            *bonus.entry(Stats::BreakEffect_).or_default() += 20.0;
                        }
                    }
                }
                "308" => {}
                "309" => {
                    if num_relics >= 2 {
                        if *base_stats
                            .get(&Stats::CritRate_)
                            .ok_or(eyre!("Missing CRIT Rate"))?
                            >= 70.0
                            && *skill_type == SkillType::BasicAttack
                            || *skill_type == SkillType::Skill
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                        }
                    }
                }
                "310" => {}
                "311" => {
                    if num_relics >= 2 {
                        if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 160.0 {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 18.0;
                        } else if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 135.0
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                        }
                    }
                }
                "312" => {}
                "313" => {
                    if num_relics >= 2 {
                        *bonus.entry(Stats::CritDmg_).or_default() +=
                            4.0 * self.config.stack_313 as f64;
                    }
                }
                "314" => {
                    if num_relics >= 2 {
                        if teammates.iter().any(|t| t.get_path() == path) {
                            *bonus.entry(Stats::CritRate_).or_default() += 12.0;
                        }
                    }
                }
                "315" => {
                    if num_relics >= 2 && *skill_type == SkillType::FollowUpAttack {
                        let num_stack = min(5, self.config.stack_315);
                        *bonus.entry(Stats::DmgBoost_).or_default() += 5.0 * num_stack as f64;
                        if num_stack == 5 {
                            *bonus.entry(Stats::CritDmg_).or_default() += 25.0
                        }
                    }
                }
                "316" => {
                    if num_relics >= 2 && self.config.activate_316 {
                        *bonus.entry(Stats::BreakEffect_).or_default() += 40.0;
                    }
                }
                "317" => {}
                "318" => {
                    if num_relics >= 2 && self.config.activate_318 {
                        *bonus.entry(Stats::CritDmg_).or_default() += 32.0;
                    }
                }
                "319" => {
                    if num_relics >= 2
                        && *base_stats.get(&Stats::Hp).ok_or(eyre!("Missing HP"))? >= 5000.0
                    {
                        *bonus.entry(Stats::CritDmg_).or_default() += 28.0;
                    }
                }
                "320" => {
                    if num_relics >= 2 {
                        if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 180.0 {
                            *bonus.entry(Stats::OutgoingHealingBoost_).or_default() += 20.0;
                        } else if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 135.0
                        {
                            *bonus.entry(Stats::OutgoingHealingBoost_).or_default() += 12.0;
                        }
                    }
                }
                "321" => {
                    if num_relics >= 2 {
                        if self.config.stack_321 > 4 {
                            *bonus.entry(Stats::DmgBoost_).or_default() +=
                                9.0 * std::cmp::min(4, self.config.stack_321) as f64;
                        } else if self.config.stack_321 < 4 {
                            *bonus.entry(Stats::DmgBoost_).or_default() +=
                                12.0 * std::cmp::min(3, 4 - self.config.stack_321) as f64;
                        }
                    }
                }
                "322" => {
                    if num_relics >= 2 && *damage_type == DamageType::DamageOnTime {
                        if *base_stats.get(&Stats::Atk).ok_or(eyre!("Missing ATK"))? >= 3600.0 {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 24.0
                        } else if *base_stats.get(&Stats::Atk).ok_or(eyre!("Missing ATK"))?
                            >= 2400.0
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0
                        }
                    }
                }
                _ => todo!(),
            }
        }
        Ok(bonus)
    }
}
