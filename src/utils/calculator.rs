use crate::{
    character::Support,
    domain::{
        AttackType, CharacterEntity, CritEnum, DamageType, Enemy, LightConeEntity, Relics,
        SkillType, Stats,
    },
};
use eyre::{eyre, Result};
use std::collections::HashMap;

pub fn calculate_stats(
    bonus: &HashMap<Stats, f64>,
    character: &CharacterEntity,
    light_cone: &Option<LightConeEntity>,
) -> HashMap<Stats, f64> {
    let hp = (character.base_hp + light_cone.as_ref().map(|lc| lc.base_hp).unwrap_or_default())
        * (1.0 + bonus.get(&Stats::Hp_).cloned().unwrap_or_default() / 100.0)
        + bonus.get(&Stats::Hp).cloned().unwrap_or_default();
    let atk = (character.base_atk
        + light_cone
            .as_ref()
            .map(|lc| lc.base_atk)
            .unwrap_or_default())
        * (1.0 + bonus.get(&Stats::Atk_).cloned().unwrap_or_default() / 100.0)
        + bonus.get(&Stats::Atk).cloned().unwrap_or_default();
    let def = (character.base_def
        + light_cone
            .as_ref()
            .map(|lc| lc.base_def)
            .unwrap_or_default())
        * (1.0 + bonus.get(&Stats::Def_).cloned().unwrap_or_default() / 100.0)
        + bonus.get(&Stats::Def).cloned().unwrap_or_default();
    let spd = character.base_spd
        * (1.0 + bonus.get(&Stats::Spd_).cloned().unwrap_or_default() / 100.0)
        + bonus.get(&Stats::Spd).cloned().unwrap_or_default();
    let crit_rate =
        character.critical_chance + bonus.get(&Stats::CritRate_).cloned().unwrap_or_default();
    let crit_dmg =
        character.critical_damage + bonus.get(&Stats::CritDmg_).cloned().unwrap_or_default();
    let energy_regen_rate = 100.0
        + bonus
            .get(&Stats::EnergyRegenerationRate_)
            .cloned()
            .unwrap_or_default();
    let effect_hit_rate = bonus
        .get(&Stats::EffectHitRate_)
        .cloned()
        .unwrap_or_default();
    let break_effect = bonus.get(&Stats::BreakEffect_).cloned().unwrap_or_default();
    let effect_res = bonus.get(&Stats::EffectRes_).cloned().unwrap_or_default();
    let outgoing_healing_boost = bonus
        .get(&Stats::OutgoingHealingBoost_)
        .cloned()
        .unwrap_or_default();
    let base_stats = HashMap::from([
        (Stats::Hp, hp),
        (Stats::Atk, atk),
        (Stats::Def, def),
        (Stats::Spd, spd),
        (Stats::CritRate_, crit_rate),
        (Stats::CritDmg_, crit_dmg),
        (Stats::EnergyRegenerationRate_, energy_regen_rate),
        (Stats::EffectHitRate_, effect_hit_rate),
        (Stats::BreakEffect_, break_effect),
        (Stats::EffectRes_, effect_res),
        (Stats::OutgoingHealingBoost_, outgoing_healing_boost),
    ]);
    base_stats
}

pub fn base_stats_and_bonus(
    character: &CharacterEntity,
    light_cone: &Option<LightConeEntity>,
    relics: &Relics,
    attack_type: &AttackType,
    skill_type: &SkillType,
    damage_type: &DamageType,
    teammates: &[Box<dyn Support>],
) -> Result<(HashMap<Stats, f64>, HashMap<Stats, f64>)> {
    let mut bonus = relics.calculate_bonus_before_battle(attack_type)?;
    for (s, b) in &character.stat_bonus {
        *bonus.entry(s.clone()).or_default() += b;
        match (attack_type, s) {
            (AttackType::Lightning, Stats::LightningDmgBoost_)
            | (AttackType::Physical, Stats::PhysicalDmgBoost_)
            | (AttackType::Wind, Stats::WindDmgBoost_)
            | (AttackType::Fire, Stats::FireDmgBoost_)
            | (AttackType::Ice, Stats::IceDmgBoost_)
            | (AttackType::Imaginary, Stats::ImaginaryDmgBoost_)
            | (AttackType::Quantum, Stats::QuantumDmgBoost_) => {
                *bonus.entry(Stats::DmgBoost_).or_default() += b
            }
            _ => (),
        }
    }
    let initial_light_cone_bonus = light_cone
        .as_ref()
        .map(|lc| lc.get_bonus_before_battle())
        .transpose()?;
    if let Some(lc_bonus) = initial_light_cone_bonus {
        for (s, b) in lc_bonus {
            *bonus.entry(s.clone()).or_default() += b;
        }
    }
    let base_stats = calculate_stats(&bonus, &character, &light_cone);
    let bonus_during_battle = relics.calculate_bonus_during_battle(
        character._character.path.clone(),
        attack_type,
        skill_type,
        damage_type,
        &base_stats,
        teammates,
    )?;
    for (stat, value) in bonus_during_battle {
        *bonus.entry(stat).or_default() += value;
    }
    let light_cone_bonus = light_cone
        .as_ref()
        .map(|lc| lc.get_bonus_during_battle(skill_type, damage_type, &base_stats))
        .transpose()?;
    if let Some(lc_bonus) = light_cone_bonus {
        for (stat, val) in lc_bonus {
            *bonus.entry(stat).or_default() += val;
        }
    }
    Ok((calculate_stats(&bonus, &character, &light_cone), bonus))
}

pub fn toughness(enemy: &Enemy) -> f64 {
    let toughness_break = match enemy.toughness_break {
        true => 1.0,
        false => 0.9,
    };
    toughness_break
}

pub fn dmg_mit(enemy: &Enemy) -> Result<f64> {
    let dmg_mitigation = if enemy.dmg_mitigation.is_empty() {
        1.0
    } else {
        let mut first = 1.0
            - enemy
                .dmg_mitigation
                .first()
                .ok_or(eyre!("Unexpected error"))?
                / 100.0;
        for dmg_mit in &enemy.dmg_mitigation[1..] {
            first *= 1.0 - dmg_mit / 100.0;
        }
        first
    };
    Ok(dmg_mitigation)
}

pub fn vul(enemy: &Enemy) -> f64 {
    let vulnerebility = 1.0 + enemy.vulnerability / 100.0;
    vulnerebility
}

pub fn res(enemy: &Enemy, bonus: &HashMap<Stats, f64>) -> f64 {
    let res = 1.0
        - ((enemy.resistance
            - bonus
                .get(&Stats::ResPenentration_)
                .cloned()
                .unwrap_or_default())
            / 100.0);
    res
}

pub fn weaken(enemy: &Enemy) -> f64 {
    let weaken = 1.0 - enemy.weaken / 100.0;
    weaken
}

pub fn dmg_boost(bonus: &HashMap<Stats, f64>) -> f64 {
    1.0 + bonus.get(&Stats::DmgBoost_).cloned().unwrap_or_default() / 100.0
}

pub fn crit_dmg(crit: CritEnum, bonus: &HashMap<Stats, f64>, character: &CharacterEntity) -> f64 {
    let crit_rate = match crit {
        CritEnum::NoCrit => 0.0,
        CritEnum::Avg => {
            (bonus.get(&Stats::CritRate_).cloned().unwrap_or_default() + character.critical_chance)
                / 100.0
        }
        CritEnum::Crit => 1.0,
    };
    let ret =
        crit_rate * (bonus.get(&Stats::CritDmg_).cloned().unwrap_or_default() / 100.0) + 1.0;
    if crit == CritEnum::NoCrit {
        1.0
    } else {
        ret
    }
}

pub fn def(enemy: &Enemy, bonus: &HashMap<Stats, f64>, character: &CharacterEntity) -> f64 {
    let denom = 1.0 + enemy.def_bonus
        - bonus
            .get(&Stats::DefReduction_)
            .cloned()
            .unwrap_or_default()
            / 100.0
        - bonus.get(&Stats::DefIgnore_).cloned().unwrap_or_default() / 100.0;
    let def = ((character._character.level + 20) as f64)
        / ((enemy.level + 20) as f64 * (if denom < 0.0 { 0.0 } else { denom })
            + (character._character.level + 20) as f64);
    def
}
