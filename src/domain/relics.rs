use eyre::{eyre, Result};
use itertools::Itertools;
use std::{cmp::min, collections::HashMap};

use super::{battle_condition::BattleConditionEnum, Relic, Stats, Tag};

#[derive(Clone, Debug)]
pub struct Relics {
    pub relics: Vec<Relic>,
}

impl Relics {
    pub fn calculate_bonus_before_battle(&self, tags: &[Tag]) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        self.calculate_bonus(tags, &mut bonus)?;
        self.calculate_set_bonus(tags, &mut bonus)?;
        Ok(bonus)
    }

    pub fn calculate_bonus(&self, tags: &[Tag], bonus: &mut HashMap<Stats, f64>) -> Result<()> {
        for relic in &self.relics {
            for substat in &relic.substats {
                *bonus.entry(substat.key.clone()).or_default() += substat.value;
            }
            if tags.contains(&Tag::Lightning) && relic.mainstat == Stats::LightningDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Wind) && relic.mainstat == Stats::WindDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Fire) && relic.mainstat == Stats::FireDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Ice) && relic.mainstat == Stats::IceDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Quantum) && relic.mainstat == Stats::QuantumDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Imaginary) && relic.mainstat == Stats::ImaginaryDmgBoost_
            {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else if tags.contains(&Tag::Physical) && relic.mainstat == Stats::PhysicalDmgBoost_ {
                *bonus.entry(Stats::DmgBoost_).or_default() += relic.get_mainstat()?;
            } else {
                *bonus.entry(relic.mainstat.clone()).or_default() += relic.get_mainstat()?;
            }
        }
        Ok(())
    }

    pub fn calculate_set_bonus(&self, tags: &[Tag], bonus: &mut HashMap<Stats, f64>) -> Result<()> {
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
                    if num_relics >= 2 && tags.contains(&Tag::Ice) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "105" => {
                    if num_relics >= 2 && tags.contains(&Tag::Physical) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "106" => {}
                "107" => {
                    if num_relics >= 2 && tags.contains(&Tag::Fire) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "108" => {
                    if num_relics >= 2 && tags.contains(&Tag::Quantum) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                    if num_relics >= 4 {
                        *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                    }
                }
                "109" => {
                    if num_relics >= 2 && tags.contains(&Tag::Lightning) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                    }
                }
                "110" => {
                    if num_relics >= 2 && tags.contains(&Tag::Wind) {
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
                    if num_relics >= 2 && tags.contains(&Tag::Imaginary) {
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
                _ => todo!(),
            }
        }
        Ok(())
    }

    pub fn calculate_bonus_during_battle(
        &self,
        tags: &[Tag],
        base_stats: &HashMap<Stats, f64>,
        battle_condition: &[BattleConditionEnum],
    ) -> Result<HashMap<Stats, f64>> {
        let mut bonus = HashMap::new();
        let count = self.relics.iter().counts_by(|r| r.set_id.clone());
        for (set_id, num_relics) in count {
            match set_id.as_str() {
                "101" => {}
                "102" => {
                    if num_relics >= 4 {
                        if tags.contains(&Tag::BasicAtk) {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 10.0;
                        }
                    }
                }
                "103" => {}
                "104" => {
                    if num_relics >= 4 {
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterUsingUltimate {
                                    number_of_turns_since_using_ultimate,
                                    ..
                                } => {
                                    if *number_of_turns_since_using_ultimate <= 2 {
                                        *bonus.entry(Stats::CritDmg_).or_default() += 25.0;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "105" => {
                    if num_relics >= 4 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterWearerIsHit {
                                    number_of_times, ..
                                } => stack += number_of_times,
                                BattleConditionEnum::AfterWearerAttack { number_of_times } => {
                                    stack += number_of_times
                                }
                                _ => (),
                            }
                        }
                        stack = min(5, stack);
                        *bonus.entry(Stats::Atk_).or_default() += 5.0 * stack as f64;
                    }
                }
                "107" => {
                    if num_relics >= 4 {
                        if tags.contains(&Tag::Skill) {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                        }
                        if tags.contains(&Tag::Fire) {
                            for condition in battle_condition {
                                match condition {
                                    BattleConditionEnum::AfterUsingUltimate {
                                        next_attack_after_ultimate,
                                        ..
                                    } => {
                                        if *next_attack_after_ultimate {
                                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                "108" => {
                    if num_relics >= 4 {
                        if battle_condition.contains(&BattleConditionEnum::EnemyHasQuantumWeakness)
                        {
                            *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                        }
                    }
                }
                "109" => {
                    if num_relics >= 4 {
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterUsingSkill {
                                    number_of_turns_since_using_the_skill,
                                } => {
                                    if *number_of_turns_since_using_the_skill <= 1 {
                                        *bonus.entry(Stats::Atk_).or_default() += 20.0;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "110" => {}
                "111" => {}
                "112" => {
                    if num_relics >= 4 {
                        if battle_condition
                            .contains(&BattleConditionEnum::AfterAttackingDebuffedEnemy)
                        {
                            *bonus.entry(Stats::CritRate_).or_default() += 10.0;
                        }
                        if battle_condition
                            .contains(&BattleConditionEnum::WhenAttackingImprisonedEnemy)
                        {
                            *bonus.entry(Stats::CritDmg_).or_default() += 20.0;
                        }
                    }
                }
                "113" => {
                    if num_relics >= 4 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterWearerIsHit {
                                    number_of_times,
                                    within_number_of_turns,
                                } => {
                                    if *within_number_of_turns <= 2 {
                                        stack += number_of_times
                                    }
                                }
                                BattleConditionEnum::WhenWearerLosingHp {
                                    number_of_times,
                                    within_number_of_turns,
                                    ..
                                } => {
                                    if *within_number_of_turns <= 2 {
                                        stack += number_of_times
                                    }
                                }
                                _ => (),
                            }
                        }
                        stack = min(2, stack);
                        *bonus.entry(Stats::CritRate_).or_default() += 8.0 * stack as f64;
                    }
                }
                "114" => {}
                "115" => {
                    if num_relics >= 2 && tags.contains(&Tag::FollowUpAtk) {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                    }
                    if num_relics >= 4 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterFollowUpAtkDealtDmg {
                                    number_of_hit_in_one_move,
                                    ..
                                } => stack += number_of_hit_in_one_move,
                                _ => (),
                            }
                        }
                        stack = min(8, stack);
                        *bonus.entry(Stats::Atk_).or_default() += 6.0 * stack as f64;
                    }
                }
                "116" => {
                    if num_relics >= 4 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::WhenAttackingEnemyWithDot {
                                    number_of_dots_enemy_has,
                                    ..
                                } => stack += number_of_dots_enemy_has,
                                _ => (),
                            }
                        }
                        stack = min(3, stack);
                        *bonus.entry(Stats::DefIgnore_).or_default() += 6.0 * stack as f64;
                    }
                }
                "117" => {
                    if num_relics >= 2
                        && battle_condition.iter().any(|c| {
                            matches!(c, BattleConditionEnum::WhenAttackingEnemyWithDebuff { .. })
                        })
                        || battle_condition.iter().any(|c| {
                            matches!(c, BattleConditionEnum::WhenAttackingEnemyWithDot { .. })
                        })
                    // DoTs are considered debuffs as well
                    {
                        *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                    }
                    if num_relics >= 4 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::WhenAttackingEnemyWithDebuff {
                                    number_of_debuffs_enemy_has,
                                    within_number_of_turns,
                                } => {
                                    if *within_number_of_turns <= 1 {
                                        stack += number_of_debuffs_enemy_has
                                    }
                                }
                                BattleConditionEnum::WhenAttackingEnemyWithDot {
                                    number_of_dots_enemy_has,
                                    within_number_of_turns,
                                    ..
                                } => {
                                    if *within_number_of_turns <= 1 {
                                        stack += number_of_dots_enemy_has
                                    }
                                }
                                _ => (),
                            }
                        }
                        if stack == 2 {
                            for condition in battle_condition {
                                match condition {
                                    BattleConditionEnum::AfterWearerInflictingDebuffs {
                                        ..
                                    } => {
                                        *bonus.entry(Stats::CritDmg_).or_default() += 16.0;
                                    }
                                    _ => {
                                        *bonus.entry(Stats::CritDmg_).or_default() += 8.0;
                                    }
                                }
                            }
                        } else if stack >= 3 {
                            for condition in battle_condition {
                                match condition {
                                    BattleConditionEnum::AfterWearerInflictingDebuffs {
                                        ..
                                    } => {
                                        *bonus.entry(Stats::CritDmg_).or_default() += 24.0;
                                    }
                                    _ => {
                                        *bonus.entry(Stats::CritDmg_).or_default() += 12.0;
                                    }
                                }
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
                            && tags.contains(&Tag::BreakDmg)
                        {
                            *bonus.entry(Stats::DefIgnore_).or_default() += 10.0;
                        } else if *base_stats
                            .get(&Stats::BreakEffect_)
                            .ok_or(eyre!("Missing break effect"))?
                            >= 250.0
                            && tags.contains(&Tag::SuperBreakDmg)
                        {
                            *bonus.entry(Stats::DefIgnore_).or_default() += 25.0;
                        }
                    }
                }
                "120" => {
                    if num_relics >= 4 {
                        if battle_condition.iter().any(|c| {
                            matches!(c, BattleConditionEnum::AfterFollowUpAtkDealtDmg { .. })
                        }) && tags.contains(&Tag::Ultimate)
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 36.0;
                        }
                    }
                }
                "121" => {}
                "122" => {
                    if num_relics >= 4 {
                        if tags.contains(&Tag::Skill) || tags.contains(&Tag::Ultimate) {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                        }
                        if tags.contains(&Tag::Skill) {
                            for condition in battle_condition {
                                match condition {
                                    BattleConditionEnum::AfterUsingUltimate {
                                        next_skill_after_ultimate,
                                        ..
                                    } => {
                                        if *next_skill_after_ultimate {
                                            *bonus.entry(Stats::DmgBoost_).or_default() += 25.0;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
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
                        if battle_condition.contains(&BattleConditionEnum::BeforeFirstAttack)
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
                            && (tags.contains(&Tag::Ultimate) || tags.contains(&Tag::FollowUpAtk))
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
                            && (tags.contains(&Tag::BasicAtk) || tags.contains(&Tag::Skill))
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 20.0;
                        }
                    }
                }
                "310" => {}
                "311" => {
                    if num_relics >= 2 {
                        if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 135.0 {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 12.0;
                        } else if *base_stats.get(&Stats::Spd).ok_or(eyre!("Missing SPD"))? >= 160.0
                        {
                            *bonus.entry(Stats::DmgBoost_).or_default() += 18.0;
                        }
                    }
                }
                "312" => {}
                "313" => {
                    if num_relics >= 2 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterEnemyDefeated {
                                    number_of_enemies_defeated,
                                    ..
                                } => stack += number_of_enemies_defeated,
                                _ => (),
                            }
                        }
                        *bonus.entry(Stats::CritDmg_).or_default() += 4.0 * stack as f64;
                    }
                }
                "314" => {
                    if num_relics >= 2 {
                        if battle_condition.iter().any(|c| {
                            matches!(c, BattleConditionEnum::TeammatesSamePathWithWearer { .. })
                        }) {
                            *bonus.entry(Stats::CritRate_).or_default() += 12.0;
                        }
                    }
                }
                "315" => {
                    if num_relics >= 2 {
                        let mut stack = 0;
                        for condition in battle_condition {
                            match condition {
                                BattleConditionEnum::AfterFollowUpAtkDealtDmg {
                                    number_of_times_allies_used_follow_up_atk,
                                    ..
                                } => stack += number_of_times_allies_used_follow_up_atk,
                                _ => (),
                            }
                        }
                        if tags.contains(&Tag::FollowUpAtk) {
                            let num_stack = min(5, stack);
                            *bonus.entry(Stats::DmgBoost_).or_default() += 5.0 * num_stack as f64;
                            if num_stack == 5 {
                                *bonus.entry(Stats::CritDmg_).or_default() += 25.0
                            }
                        }
                    }
                }
                "316" => {
                    if num_relics >= 2 {
                        if battle_condition.contains(&BattleConditionEnum::EnemyHasFireWeakness) {
                            *bonus.entry(Stats::BreakEffect_).or_default() += 40.0;
                        }
                    }
                }
                "317" => {}
                "318" => {
                    if battle_condition.contains(&BattleConditionEnum::WearerSummonOnField) {
                        *bonus.entry(Stats::CritDmg_).or_default() += 32.0;
                    }
                }
                _ => todo!(),
            }
        }
        Ok(bonus)
    }
}
