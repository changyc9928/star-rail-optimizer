use crate::domain::{Character, CombatTypes, LightCone, Relic, RelicSetName, Stats};
use eval::Expr;
use eyre::Result;
use itertools::Itertools;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Clone)]
pub struct Evaluator {
    pub character: Character,
    pub light_cone: LightCone,
    pub constraint: HashMap<Stats, f64>,
    pub set_bonus: HashMap<RelicSetName, HashMap<u8, (Stats, f64)>>,
    pub conditional_set_bonus_by_value: HashMap<RelicSetName, HashMap<u8, (Stats, Stats, f64)>>,
    pub other_bonus: HashMap<Stats, f64>,
    pub base_stats_formulas: HashMap<Stats, String>,
    pub target_formula: String,
    pub target_name: String,
}

impl Evaluator {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        character: Character,
        light_cone: LightCone,
        constraint: HashMap<Stats, f64>,
        set_bonus: HashMap<RelicSetName, HashMap<u8, (Stats, f64)>>,
        conditional_set_bonus_by_value: HashMap<RelicSetName, HashMap<u8, (Stats, Stats, f64)>>,
        other_bonus: HashMap<Stats, f64>,
        base_stats_formulas: HashMap<Stats, String>,
        target_formula: &str,
        target_name: &str,
    ) -> Self {
        Self {
            character,
            light_cone,
            constraint,
            set_bonus,
            conditional_set_bonus_by_value,
            other_bonus,
            base_stats_formulas,
            target_formula: target_formula.to_owned(),
            target_name: target_name.to_owned(),
        }
    }

    pub fn calculate_total(&self, relics: Vec<Relic>) -> Result<HashMap<Stats, f64>> {
        let mut totals = HashMap::new();
        for stats in Stats::iter() {
            let mut values = 0.0;
            relics.iter().for_each(|relic| {
                if relic.mainstat == stats {
                    values += relic.get_mainstat();
                }
                values += relic
                    .substats
                    .iter()
                    .filter_map(|s| if s.key == stats { Some(s.value) } else { None })
                    .sum::<f64>();
            });
            totals.insert(stats, values);
        }
        let count = relics.iter().counts_by(|relic| relic.set.clone());
        for (set, num) in count {
            let set_bonus = self
                .set_bonus
                .get(&set)
                .and_then(|s| s.get(&(num as u8)))
                .cloned();
            if let Some((stat, bonus_value)) = set_bonus {
                totals
                    .entry(stat.clone())
                    .and_modify(|value| *value += bonus_value);
            }
        }
        for (stats, bonus) in &self.other_bonus {
            totals
                .entry(stats.clone())
                .and_modify(|value| *value += bonus);
        }
        Ok(totals)
    }

    pub fn evaluate(&self, relics: Vec<Relic>) -> Result<f64> {
        let totals = self.calculate_total(relics)?;
        let base_stats: HashMap<Stats, f64> = self
            .base_stats_formulas
            .iter()
            .map(|(base_stats, formula)| {
                Ok::<(Stats, f64), eyre::Report>((
                    base_stats.clone(),
                    serde_json::from_value(
                        Expr::new(formula)
                            .value("Character_HP", self.character.base_hp)
                            .value("Character_ATK", self.character.base_atk)
                            .value("Character_DEF", self.character.base_def)
                            .value("Character_SPD", self.character.base_spd)
                            .value("LightCone_HP", self.light_cone.light_cone_stats.hp)
                            .value("LightCone_ATK", self.light_cone.light_cone_stats.atk)
                            .value("LightCone_DEF", self.light_cone.light_cone_stats.def)
                            .value("Additive_HP_Bonus", totals.get(&Stats::Hp))
                            .value("Percentage_HP_Bonus", totals.get(&Stats::Hp_))
                            .value("Additive_ATK_Bonus", totals.get(&Stats::Atk))
                            .value("Percentage_ATK_Bonus", totals.get(&Stats::Atk_))
                            .value("Additive_DEF_Bonus", totals.get(&Stats::Def))
                            .value("Percentage_DEF_Bonus", totals.get(&Stats::Def_))
                            .value(
                                "Percentage_DEF_Reduction",
                                totals.get(&Stats::DefReduction_),
                            )
                            .value("Additive_SPD_Bonus", totals.get(&Stats::Spd))
                            .value("Percentage_SPD_Bonus", totals.get(&Stats::Spd_))
                            .value(
                                "DMG_Boost",
                                totals
                                    .get(match self.character.combat_type {
                                        Some(CombatTypes::Fire) => &Stats::FireDMGBoost_,
                                        Some(CombatTypes::Wind) => &Stats::WindDMGBoost_,
                                        Some(CombatTypes::Ice) => &Stats::IceDMGBoost_,
                                        Some(CombatTypes::Lightning) => &Stats::LightningDMGBoost_,
                                        Some(CombatTypes::Physical) => &Stats::PhysicalDMGBoost_,
                                        Some(CombatTypes::Quantum) => &Stats::QuantumDMGBoost_,
                                        Some(CombatTypes::Imaginary) => &Stats::ImaginaryDMGBoost_,
                                        None => eyre::bail!("A character must have combat type"),
                                    })
                                    .and_then(|total| {
                                        totals
                                            .get(&Stats::CommonDMGBoost_)
                                            .map(|common| common + total)
                                    }),
                            )
                            .value("CRIT_Rate", totals.get(&Stats::CritRate_))
                            .value("CRIT_DMG", totals.get(&Stats::CritDmg_))
                            .value("Break_Effect", totals.get(&Stats::BreakEffect_))
                            .value("Effect_Hit_Rate", totals.get(&Stats::EffectHitRate_))
                            .value("Effect_RES", totals.get(&Stats::EffectRES_))
                            .value(
                                "Energy_Regeneration_Rate",
                                totals.get(&Stats::EnergyRegenerationRate_),
                            )
                            .exec()?,
                    )?,
                ))
            })
            .collect::<Result<_>>()?;
        let expression = Expr::new(self.target_formula.clone())
            .value("Character_HP", self.character.base_hp)
            .value("LightCone_HP", self.light_cone.light_cone_stats.hp)
            .value("Percentage_HP_Bonus", totals.get(&Stats::Hp_))
            .value("Additive_HP_Bonus", totals.get(&Stats::Hp))
            .exec()?;
        // TODO: check with the constraints
        Ok(serde_json::from_value(expression)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        CharacterName, CharacterSkills, CharacterTraces, LightConeName, LightConeStats, Slot,
        SubStats,
    };

    #[tokio::test]
    async fn test_evaluation() -> Result<()> {
        let mut fu_xuan = Character::new(
            CharacterName::FuXuan,
            80,
            6,
            0,
            CharacterSkills {
                basic: 1,
                skill: 9,
                ult: 9,
                talent: 9,
            },
            CharacterTraces {
                ability_1: true,
                ability_2: true,
                ability_3: true,
                stat_1: true,
                stat_2: true,
                stat_3: false,
                stat_4: true,
                stat_5: true,
                stat_6: true,
                stat_7: false,
                stat_8: true,
                stat_9: true,
                stat_10: false,
                total_bonus: HashMap::new(),
            },
            None,
            vec![],
        )
        .await?;
        let mut we_are_wild_fire = LightCone {
            key: LightConeName::WeAreWildfire,
            level: 50,
            ascension: 3,
            superimposition: 0,
            location: None,
            lock: false,
            light_cone_stats: LightConeStats {
                ..Default::default()
            },
            _id: "lightcone_100".to_string(),
        };
        let head = Relic {
            set: RelicSetName::KnightOfPurityPalace,
            slot: Slot::Head,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp,
            substats: vec![
                SubStats {
                    key: Stats::Atk,
                    value: 38.0,
                },
                SubStats {
                    key: Stats::Def,
                    value: 21.0,
                },
                SubStats {
                    key: Stats::Hp_,
                    value: 13.8,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 7.3,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_1".to_string(),
        };
        let hands = Relic {
            set: RelicSetName::LongevousDisciple,
            slot: Slot::Hands,
            rarity: 5,
            level: 15,
            mainstat: Stats::Atk,
            substats: vec![
                SubStats {
                    key: Stats::Hp_,
                    value: 8.2,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 7.7,
                },
                SubStats {
                    key: Stats::CritDmg_,
                    value: 11.6,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 7.3,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_2".to_string(),
        };
        let body = Relic {
            set: RelicSetName::LongevousDisciple,
            slot: Slot::Body,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Hp,
                    value: 80.0,
                },
                SubStats {
                    key: Stats::Atk,
                    value: 16.0,
                },
                SubStats {
                    key: Stats::Def,
                    value: 21.0,
                },
                SubStats {
                    key: Stats::Def_,
                    value: 19.9,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_3".to_string(),
        };
        let feet = Relic {
            set: RelicSetName::KnightOfPurityPalace,
            slot: Slot::Feet,
            rarity: 5,
            level: 15,
            mainstat: Stats::Spd,
            substats: vec![
                SubStats {
                    key: Stats::Atk,
                    value: 19.0,
                },
                SubStats {
                    key: Stats::Hp_,
                    value: 7.3,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 4.3,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 16.4,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_4".to_string(),
        };
        let sphere = Relic {
            set: RelicSetName::FleetOfTheAgeless,
            slot: Slot::PlanarSphere,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Def_,
                    value: 9.7,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 7.7,
                },
                SubStats {
                    key: Stats::EffectRES_,
                    value: 12.0,
                },
                SubStats {
                    key: Stats::BreakEffect_,
                    value: 5.8,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_5".to_string(),
        };
        let rope = Relic {
            set: RelicSetName::FleetOfTheAgeless,
            slot: Slot::LinkRope,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Def,
                    value: 35.0,
                },
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
                    value: 5.8,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_6".to_string(),
        };

        let relics = vec![head, hands, body, feet, sphere, rope];
        fu_xuan.add_base_stats().await?;
        we_are_wild_fire.get_main_stat().await?;

        let trace_bonus = fu_xuan.traces.total_bonus.clone();
        let set_bonus = HashMap::from([
            (
                RelicSetName::KnightOfPurityPalace,
                HashMap::from([(2, (Stats::Def_, 15.0))]),
            ),
            (
                RelicSetName::LongevousDisciple,
                HashMap::from([(2, (Stats::Hp_, 12.0))]),
            ),
            (
                RelicSetName::FleetOfTheAgeless,
                HashMap::from([(2, (Stats::Hp_, 12.0))]),
            ),
        ]);

        let hp_formula =
            "(Character_HP + LightCone_HP) * (1 + Percentage_HP_Bonus / 100) + Additive_HP_Bonus";
        let atk_formula =
            "(Character_ATK + LightCone_ATK) * (1 + Percentage_ATK_Bonus / 100) + Additive_ATK_Bonus";
        let def_formula =
            "(Character_DEF + LightCone_DEF) * (1 + (Percentage_DEF_Bonus - Percentage_DEF_Reduction) / 100) + Additive_DEF_Bonus";

        let evaluator = Evaluator::new(
            fu_xuan,
            we_are_wild_fire,
            HashMap::new(),
            set_bonus,
            HashMap::new(),
            trace_bonus,
            HashMap::from([
                (Stats::Hp, hp_formula.to_owned()),
                (Stats::Atk, atk_formula.to_owned()),
                (Stats::Def, def_formula.to_owned()),
            ]),
            hp_formula,
            "Maximum HP",
        );

        let result = evaluator.evaluate(relics)?;
        assert_eq!(result, 6512.039936000001);
        Ok(())
    }
}
