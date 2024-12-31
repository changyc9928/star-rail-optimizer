use crate::domain::{
    BattleConditionEnum, CharacterEntity, CritEnum, Enemy, LightConeEntity, Relics, Stats, Tag,
};
use eyre::{bail, eyre, Result};
use std::collections::HashMap;

use super::Evaluator;

pub struct Acheron {
    pub character: CharacterEntity,
    pub light_cone: Option<LightConeEntity>,
}

impl Evaluator for Acheron {
    fn evaluate(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        target: &str,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        match target {
            "RAINBLADE" => self.rainblade(&relics, &enemy, battle_conditions),
            "CRIMSON_KNOT" => self.crimson_knot(&relics, &enemy, battle_conditions),
            "STYGIAN_RESURGE" => self.stygian_resurge(&relics, &enemy, battle_conditions),
            "THUNDER_CORE" => self.thunder_core(&relics, &enemy, battle_conditions),
            "FULL_ULTIMATE_ON_THREE_ENEMIES" => {
                self.full_ultimate_multiplier_on_three_enemies(&relics, &enemy, battle_conditions)
            }
            _ => todo!(),
        }
    }
}

impl Acheron {
    fn toughness(&self, battle_conditions: &Vec<BattleConditionEnum>) -> f64 {
        let mut toughness_break = false;
        for condition in battle_conditions {
            match condition {
                BattleConditionEnum::ToughnessBreak(broken) => toughness_break = *broken,
                _ => (),
            }
        }
        let toughness_break = match toughness_break {
            true => 1.0,
            false => 0.9,
        };
        toughness_break
    }

    fn dmg_mit(&self, enemy: &Enemy) -> Result<f64, eyre::Error> {
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

    fn vul(&self, bonus: &HashMap<Stats, f64>) -> f64 {
        let vulnerebility = 1.0
            + bonus
                .get(&Stats::Vulnerebility_)
                .cloned()
                .unwrap_or_default()
                / 100.0;
        vulnerebility
    }

    fn res(&self, enemy: &Enemy, bonus: &HashMap<Stats, f64>) -> f64 {
        let res = 1.0
            - (enemy.resistance - bonus.get(&Stats::ResPenalty_).cloned().unwrap_or_default())
                / 100.0;
        res
    }

    fn weaken(&self, bonus: &HashMap<Stats, f64>) -> f64 {
        let weaken = 1.0 - bonus.get(&Stats::Weaken_).cloned().unwrap_or_default() / 100.0;
        weaken
    }

    fn dmg_boost(
        &self,
        bonus: &HashMap<Stats, f64>,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> f64 {
        1.0 + bonus.get(&Stats::DmgBoost_).cloned().unwrap_or_default() / 100.0
            + self.crinsom_knot_bonus(battle_conditions)
    }

    pub fn base_stats_and_bonus(
        &self,
        relics: &Relics,
        tags: &Vec<Tag>,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<(HashMap<Stats, f64>, HashMap<Stats, f64>)> {
        let mut bonus = relics.calculate_bonus_before_battle(&tags)?;
        for (s, b) in &self.character.stat_bonus {
            *bonus.entry(s.clone()).or_default() += b;
        }
        let base_stats = self.calculate_stats(&bonus);
        let bonus_during_battle =
            relics.calculate_bonus_during_battle(&tags, &base_stats, &battle_conditions)?;
        for (stat, value) in bonus_during_battle {
            *bonus.entry(stat).or_default() += value;
        }
        let light_cone_bonus = self
            .light_cone
            .as_ref()
            .map(|lc| lc.get_bonus(&tags, &battle_conditions))
            .transpose()?;
        if let Some(lc_bonus) = light_cone_bonus {
            for (stat, val) in lc_bonus {
                *bonus.entry(stat).or_default() += val;
            }
        }
        Ok((self.calculate_stats(&bonus), bonus))
    }

    fn calculate_stats(&self, bonus: &HashMap<Stats, f64>) -> HashMap<Stats, f64> {
        let hp = (self.character.base_hp
            + self
                .light_cone
                .as_ref()
                .map(|lc| lc.base_hp)
                .unwrap_or_default())
            * (1.0 + bonus.get(&Stats::Hp_).cloned().unwrap_or_default() / 100.0)
            + bonus.get(&Stats::Hp).cloned().unwrap_or_default();
        let atk = (self.character.base_atk
            + self
                .light_cone
                .as_ref()
                .map(|lc| lc.base_atk)
                .unwrap_or_default())
            * (1.0 + bonus.get(&Stats::Atk_).cloned().unwrap_or_default() / 100.0)
            + bonus.get(&Stats::Atk).cloned().unwrap_or_default();
        let def = (self.character.base_def
            + self
                .light_cone
                .as_ref()
                .map(|lc| lc.base_def)
                .unwrap_or_default())
            * (1.0 + bonus.get(&Stats::Def_).cloned().unwrap_or_default() / 100.0)
            + bonus.get(&Stats::Def).cloned().unwrap_or_default();
        let spd = self.character.base_spd
            * (1.0 + bonus.get(&Stats::Spd_).cloned().unwrap_or_default() / 100.0)
            + bonus.get(&Stats::Spd).cloned().unwrap_or_default();
        let crit_rate = self.character.critical_chance
            + bonus.get(&Stats::CritRate_).cloned().unwrap_or_default();
        let crit_dmg = self.character.critical_damage
            + bonus.get(&Stats::CritDmg_).cloned().unwrap_or_default();
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

    fn rainblade(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        let tags = vec![Tag::Lightning, Tag::Ultimate];
        let (base_stats, mut bonus) =
            self.base_stats_and_bonus(relics, &tags, battle_conditions)?;
        let ability_multiplier = [
            0.0, 0.1440, 0.1536, 0.1632, 0.1728, 0.1824, 0.1920, 0.2040, 0.2160, 0.2280, 0.2400,
            0.2496, 0.2592, 0.2688, 0.2784, 0.2880,
        ][self.character._character.skills.ult as usize];
        let the_abyss = self.the_abyss_multiplier(battle_conditions);
        let base_dmg = ability_multiplier
            * base_stats.get(&Stats::Atk).cloned().unwrap_or_default()
            * the_abyss;
        let crit = self.crit_dmg(battle_conditions, &bonus);
        let dmg_boost = self.dmg_boost(&bonus, battle_conditions);
        let weaken = self.weaken(&bonus);
        let def = self.def(enemy, &bonus);
        *bonus.entry(Stats::ResPenalty_).or_default() += self.talent();
        let res = self.res(enemy, &bonus);
        let vul = self.vul(&bonus);
        let dmg_mit = self.dmg_mit(enemy)?;
        let broken = self.toughness(battle_conditions);
        Ok(base_dmg * crit * dmg_boost * weaken * def * res * vul * dmg_mit * broken)
    }

    fn the_abyss_multiplier(&self, battle_conditions: &Vec<BattleConditionEnum>) -> f64 {
        let mut num_same_path = 0;
        for condition in battle_conditions {
            match condition {
                BattleConditionEnum::TeammatesSamePathWithWearer {
                    number_of_teammates_having_same_path: number_of_teammated_having_same_path,
                } => num_same_path += number_of_teammated_having_same_path,
                _ => (),
            }
        }
        if self.character._character.eidolon == 6 {
            num_same_path += 1;
        }
        let the_abyss = match self.character._character.traces.ability_2 {
            true => [1.0, 1.15, 1.6][num_same_path as usize],
            false => 1.0,
        };
        the_abyss
    }

    fn crimson_knot(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        let (base_stats, mut bonus) = self.base_stats_and_bonus(
            relics,
            &vec![Tag::Lightning, Tag::Ultimate],
            battle_conditions,
        )?;
        let ability_multiplier = [
            0.0, 0.0900, 0.0960, 0.1020, 0.1080, 0.1140, 0.1200, 0.1275, 0.1350, 0.1425, 0.1500,
            0.1560, 0.1620, 0.1680, 0.1740, 0.1800,
        ][self.character._character.skills.ult as usize];
        let mut crimson_knot = 0;
        for condition in battle_conditions {
            match condition {
                BattleConditionEnum::HittingEnemyWithCrimsonKnot {
                    number_of_crinsom_knot_enemy_has,
                } => crimson_knot += number_of_crinsom_knot_enemy_has,
                _ => (),
            }
        }
        let ability_multiplier = match crimson_knot {
            1 | 2 | 3 => ability_multiplier + ability_multiplier * crimson_knot as f64,
            0 => ability_multiplier,
            _ => bail!("Invalid number of stack of Crimson Knot"),
        };
        let the_abyss = self.the_abyss_multiplier(battle_conditions);
        let base_dmg = ability_multiplier
            * base_stats.get(&Stats::Atk).cloned().unwrap_or_default()
            * the_abyss;
        let crit = self.crit_dmg(battle_conditions, &bonus);
        let dmg_boost = self.dmg_boost(&bonus, battle_conditions);
        let weaken = self.weaken(&bonus);
        let def = self.def(enemy, &bonus);
        *bonus.entry(Stats::ResPenalty_).or_default() += self.talent();
        let res = self.res(enemy, &bonus);
        let vul = self.vul(&bonus);
        let dmg_mit = self.dmg_mit(enemy)?;
        let broken = self.toughness(battle_conditions);
        Ok(base_dmg * crit * dmg_boost * weaken * def * res * vul * dmg_mit * broken)
    }

    fn stygian_resurge(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        let (base_stats, mut bonus) = self.base_stats_and_bonus(
            relics,
            &vec![Tag::Lightning, Tag::Ultimate],
            battle_conditions,
        )?;
        let ability_multiplier = [
            0.0, 0.7200, 0.7680, 0.8160, 0.8640, 0.9120, 0.9600, 1.0200, 1.0800, 1.1400, 1.2000,
            1.2480, 1.2960, 1.3440, 1.3920, 1.4400,
        ][self.character._character.skills.ult as usize];
        let the_abyss = self.the_abyss_multiplier(battle_conditions);
        let base_dmg = ability_multiplier
            * base_stats.get(&Stats::Atk).cloned().unwrap_or_default()
            * the_abyss;
        let crit = self.crit_dmg(battle_conditions, &bonus);
        let dmg_boost = self.dmg_boost(&bonus, battle_conditions);
        let weaken = self.weaken(&bonus);
        let def = self.def(enemy, &bonus);
        *bonus.entry(Stats::ResPenalty_).or_default() += self.talent();
        let res = self.res(enemy, &bonus);
        let vul = self.vul(&bonus);
        let dmg_mit = self.dmg_mit(enemy)?;
        let broken = self.toughness(battle_conditions);
        Ok(base_dmg * crit * dmg_boost * weaken * def * res * vul * dmg_mit * broken)
    }

    fn thunder_core(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        let (base_stats, mut bonus) = self.base_stats_and_bonus(
            relics,
            &vec![Tag::Lightning, Tag::Ultimate],
            battle_conditions,
        )?;
        let ability_multiplier = if self.character._character.traces.ability_3 {
            0.25
        } else {
            0.0
        };
        let the_abyss = self.the_abyss_multiplier(battle_conditions);
        let base_dmg = ability_multiplier
            * base_stats.get(&Stats::Atk).cloned().unwrap_or_default()
            * the_abyss;
        let crit = self.crit_dmg(battle_conditions, &bonus);
        let dmg_boost = self.dmg_boost(&bonus, battle_conditions);
        let weaken = self.weaken(&bonus);
        let def = self.def(enemy, &bonus);
        *bonus.entry(Stats::ResPenalty_).or_default() += self.talent();
        let res = self.res(enemy, &bonus);
        let vul = self.vul(&bonus);
        let dmg_mit = self.dmg_mit(enemy)?;
        let broken = self.toughness(battle_conditions);
        Ok(base_dmg * crit * dmg_boost * weaken * def * res * vul * dmg_mit * broken)
    }

    fn full_ultimate_multiplier_on_three_enemies(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        battle_conditions: &Vec<BattleConditionEnum>,
    ) -> Result<f64> {
        let rainblade = self.rainblade(relics, enemy, battle_conditions)?;
        let crinsom_knot = self.crimson_knot(relics, enemy, battle_conditions)?;
        let bounce_atk = self.thunder_core(relics, enemy, battle_conditions)?;
        let stygian_resurge = self.stygian_resurge(relics, enemy, battle_conditions)?;
        Ok(rainblade * 3.0 + crinsom_knot * 9.0 + bounce_atk * 6.0 + stygian_resurge * 3.0)
    }

    fn crinsom_knot_bonus(&self, battle_conditions: &Vec<BattleConditionEnum>) -> f64 {
        let mut crinsom_knot_bonus_stack = 0;
        for condition in battle_conditions {
            match condition {
                BattleConditionEnum::AfterHittingEnemyWithCrinsomKnot { number_of_times } => {
                    crinsom_knot_bonus_stack += number_of_times
                }
                _ => (),
            }
        }
        if self.character._character.traces.ability_3 {
            return 0.3 * std::cmp::min(3, crinsom_knot_bonus_stack) as f64;
        } else {
            return 0.0;
        }
    }

    fn talent(&self) -> f64 {
        let scale = vec![
            0.0, 10.00, 11.00, 12.00, 13.00, 14.00, 15.00, 16.25, 17.50, 18.75, 20.00, 21.00,
            22.00, 23.00, 24.00, 25.00,
        ];
        scale[self.character._character.skills.talent as usize]
    }

    fn crit_dmg(
        &self,
        battle_conditions: &Vec<BattleConditionEnum>,
        bonus: &HashMap<Stats, f64>,
    ) -> f64 {
        let mut crit = CritEnum::Avg;
        for condition in battle_conditions {
            match condition {
                BattleConditionEnum::CriticalHit(crit_enum) => crit = crit_enum.clone(),
                _ => (),
            }
        }
        let crit_rate = match crit {
            CritEnum::NoCrit => 0.0,
            CritEnum::Avg => {
                (bonus.get(&Stats::CritRate_).cloned().unwrap_or_default()
                    + self.character.critical_chance)
                    / 100.0
            }
            CritEnum::Crit => 1.0,
        };
        let ret = crit_rate
            * (1.0
                + (bonus.get(&Stats::CritDmg_).cloned().unwrap_or_default()
                    + self.character.critical_damage)
                    / 100.0);
        if crit == CritEnum::NoCrit {
            1.0
        } else {
            ret
        }
    }

    fn def(&self, enemy: &Enemy, bonus: &HashMap<Stats, f64>) -> f64 {
        let def = 1.0
            - ((self.character._character.level + 20) as f64)
                / ((enemy.level + 20) as f64
                    * (1.0
                        - bonus
                            .get(&Stats::DefReduction_)
                            .cloned()
                            .unwrap_or_default()
                            / 100.0
                        - bonus.get(&Stats::DefIgnore_).cloned().unwrap_or_default() / 100.0)
                    + (self.character._character.level + 20) as f64);
        def
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        Character, CharacterSkills, CharacterTraces, LightCone, Path, Relic, Slot, SubStats,
    };

    use super::*;

    #[test]
    fn test_rainblade() -> Result<()> {
        let (acheron, relics, enemy, battle_conditions) = setup();
        let target = "RAINBLADE".to_string();

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &battle_conditions)?,
            4257.843767583633
        );
        Ok(())
    }

    #[test]
    fn test_crimson_knot() -> Result<()> {
        let (acheron, relics, enemy, battle_conditions) = setup();
        let target = "CRIMSON_KNOT".to_string();

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &battle_conditions)?,
            10644.609418959079
        );
        Ok(())
    }

    #[test]
    fn test_stygian_resurge() -> Result<()> {
        let (acheron, relics, enemy, battle_conditions) = setup();
        let target = "STYGIAN_RESURGE".to_string();

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &battle_conditions)?,
            21289.218837918157
        );
        Ok(())
    }

    #[test]
    fn test_thunder_core() -> Result<()> {
        let (acheron, relics, enemy, battle_conditions) = setup();
        let target = "THUNDER_CORE".to_string();

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &battle_conditions)?,
            4668.68834164872
        );
        Ok(())
    }

    fn setup() -> (Acheron, Relics, Enemy, Vec<BattleConditionEnum>) {
        let character = CharacterEntity {
            base_hp: 1125.43,
            base_atk: 698.54,
            base_def: 436.59,
            base_spd: 101.00,
            _base_aggro: 100,
            critical_chance: 5.0,
            critical_damage: 50.0,
            stat_bonus: HashMap::from([
                (Stats::LightningDmgBoost_, 8.0),
                (Stats::CritDmg_, 24.0),
                (Stats::Atk_, 28.0),
            ]),
            _character: Character {
                id: "1308".to_string(),
                name: "Acheron".to_string(),
                path: Path::Nihility,
                level: 80,
                ascension: 6,
                eidolon: 0,
                skills: CharacterSkills {
                    basic: 1,
                    skill: 9,
                    ult: 9,
                    talent: 9,
                },
                traces: CharacterTraces {
                    ability_1: true,
                    ability_2: true,
                    ability_3: true,
                    stat_1: true,
                    stat_2: true,
                    stat_3: true,
                    stat_4: true,
                    stat_5: true,
                    stat_6: true,
                    stat_7: true,
                    stat_8: true,
                    stat_9: true,
                    stat_10: true,
                },
            },
        };
        let light_cone = LightConeEntity {
            base_hp: 952.56,
            base_atk: 476.28,
            base_def: 330.75,
            _light_cone: LightCone {
                id: "21001".to_string(),
                name: "Good Night and Sleep Well".to_string(),
                level: 80,
                ascension: 6,
                superimposition: 1,
                location: Some("1308".to_string()),
                lock: true,
                _uid: "light_cone_1".to_string(),
            },
        };
        let relics = Relics {
            relics: vec![
                Relic {
                    set_id: "117".to_string(),
                    name: "Pioneer's Heatproof Shell".to_string(),
                    slot: Slot::Head,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::Hp,
                    substats: vec![
                        SubStats {
                            key: Stats::Atk_,
                            value: 12.5,
                        },
                        SubStats {
                            key: Stats::Def_,
                            value: 14.0,
                        },
                        SubStats {
                            key: Stats::CritRate_,
                            value: 3.2,
                        },
                        SubStats {
                            key: Stats::CritDmg_,
                            value: 5.1,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_1".to_string(),
                },
                Relic {
                    set_id: "117".to_string(),
                    name: "Pioneer's Lacuna Compass".to_string(),
                    slot: Slot::Hands,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::Atk,
                    substats: vec![
                        SubStats {
                            key: Stats::Def_,
                            value: 4.3,
                        },
                        SubStats {
                            key: Stats::Spd,
                            value: 6.0,
                        },
                        SubStats {
                            key: Stats::CritRate_,
                            value: 8.7,
                        },
                        SubStats {
                            key: Stats::CritDmg_,
                            value: 11.6,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_2".to_string(),
                },
                Relic {
                    set_id: "117".to_string(),
                    name: "Pioneer's Sealed Lead Apron".to_string(),
                    slot: Slot::Body,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::CritDmg_,
                    substats: vec![
                        SubStats {
                            key: Stats::Def_,
                            value: 5.4,
                        },
                        SubStats {
                            key: Stats::CritRate_,
                            value: 9.3,
                        },
                        SubStats {
                            key: Stats::EffectRes_,
                            value: 7.7,
                        },
                        SubStats {
                            key: Stats::BreakEffect_,
                            value: 11.6,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_3".to_string(),
                },
                Relic {
                    set_id: "117".to_string(),
                    name: "Pioneer's Starfaring Anchor".to_string(),
                    slot: Slot::Feet,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::Atk_,
                    substats: vec![
                        SubStats {
                            key: Stats::CritRate_,
                            value: 9.3,
                        },
                        SubStats {
                            key: Stats::CritDmg_,
                            value: 5.1,
                        },
                        SubStats {
                            key: Stats::EffectHitRate_,
                            value: 11.6,
                        },
                        SubStats {
                            key: Stats::BreakEffect_,
                            value: 5.1,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_4".to_string(),
                },
                Relic {
                    set_id: "314".to_string(),
                    name: "Izumo's Magatsu no Morokami".to_string(),
                    slot: Slot::PlanarSphere,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::LightningDmgBoost_,
                    substats: vec![
                        SubStats {
                            key: Stats::CritRate_,
                            value: 5.8,
                        },
                        SubStats {
                            key: Stats::CritDmg_,
                            value: 10.3,
                        },
                        SubStats {
                            key: Stats::EffectRes_,
                            value: 4.3,
                        },
                        SubStats {
                            key: Stats::BreakEffect_,
                            value: 18.1,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_5".to_string(),
                },
                Relic {
                    set_id: "314".to_string(),
                    name: "Izumo's Blades of Origin and End".to_string(),
                    slot: Slot::LinkRope,
                    rarity: 5,
                    level: 15,
                    mainstat: Stats::Atk_,
                    substats: vec![
                        SubStats {
                            key: Stats::Hp,
                            value: 118.0,
                        },
                        SubStats {
                            key: Stats::Atk,
                            value: 38.0,
                        },
                        SubStats {
                            key: Stats::Def,
                            value: 40.0,
                        },
                        SubStats {
                            key: Stats::CritDmg_,
                            value: 11.0,
                        },
                    ],
                    location: Some("1308".to_string()),
                    lock: true,
                    discard: false,
                    _uid: "relic_6".to_string(),
                },
            ],
        };
        let enemy = Enemy {
            level: 82,
            resistance: 0.0,
            dmg_mitigation: vec![],
        };
        let battle_conditions = vec![
            BattleConditionEnum::AfterUsingSkill,
            BattleConditionEnum::AfterUsingUltimate,
            BattleConditionEnum::AfterWearerAttack { number_of_times: 3 },
            BattleConditionEnum::AfterWearerIsHit { number_of_times: 2 },
            BattleConditionEnum::AfterAttackingDebuffedEnemy,
            BattleConditionEnum::AfterWearerInflictingDebuffs,
            BattleConditionEnum::WhenAttackingEnemyWithDebuff {
                number_of_debuffs_enemy_has: 3,
            },
            BattleConditionEnum::TeammatesSamePathWithWearer {
                number_of_teammates_having_same_path: 1,
            },
            BattleConditionEnum::HittingEnemyWithCrimsonKnot {
                number_of_crinsom_knot_enemy_has: 3,
            },
            BattleConditionEnum::CriticalHit(CritEnum::Crit),
            BattleConditionEnum::ToughnessBreak(true),
            BattleConditionEnum::AfterHittingEnemyWithCrinsomKnot { number_of_times: 3 },
        ];
        let acheron = Acheron {
            character,
            light_cone: Some(light_cone),
        };
        (acheron, relics, enemy, battle_conditions)
    }
}
