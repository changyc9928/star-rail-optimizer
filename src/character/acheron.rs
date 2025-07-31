use super::Evaluator;
use crate::{
    character::Support,
    domain::{
        AttackType, Character, CritEnum, DamageType, Enemy, LightConeEntity, Relics, SkillType,
        Stats,
    },
    utils::calculator::{
        base_stats_and_bonus, crit_dmg, def, dmg_boost, dmg_mit, res, toughness, vul, weaken,
    },
};
use eyre::Result;
use std::collections::HashMap;

pub struct Acheron {
    pub character: Character,
    pub light_cone: Option<LightConeEntity>,
    pub crimson_knot: u8,
    pub thunder_core_bonus_stack: u8,
    pub crit: CritEnum,
    pub activate_eidolon_1: bool,
}

#[derive(strum_macros::Display)]
pub enum AcheronEvaluationTarget {
    Skill,
    UltimateSingle,
    UltimateAoe,
}

impl Evaluator for Acheron {
    type Target = AcheronEvaluationTarget;

    fn evaluate(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        target: &Self::Target,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        match target {
            AcheronEvaluationTarget::UltimateSingle => {
                self.full_ultimate_multiplier_on_single_enemy(&relics, &enemy, teammates)
            }
            AcheronEvaluationTarget::UltimateAoe => {
                self.full_ultimate_multiplier_on_three_enemies(&relics, &enemy, teammates)
            }
            AcheronEvaluationTarget::Skill => self.skill(relics, enemy, teammates),
        }
    }
}

impl Acheron {
    fn calculate_damage(
        &self,
        teammates: &[Box<dyn Support>],
        ability_multiplier: f64,
        base_stats: &mut HashMap<Stats, f64>,
        bonus: &HashMap<Stats, f64>,
        enemy: &Enemy,
    ) -> Result<f64> {
        let the_abyss = self.the_abyss_multiplier(teammates);
        let base_dmg = ability_multiplier
            * base_stats.get(&Stats::Atk).cloned().unwrap_or_default()
            * the_abyss;
        *base_stats.entry(Stats::CritRate_).or_default() += self.eidolon_1();
        let crit = crit_dmg(self.crit, &base_stats, &self.character);
        let dmg_boost = dmg_boost(&bonus);
        let def = def(enemy, &bonus, &self.character);
        let res = res(enemy, &bonus);
        let vul = vul(enemy);
        let dmg_mit = dmg_mit(enemy)?;
        let weaken = weaken(enemy);
        let broken = toughness(enemy);
        Ok(base_dmg * crit * dmg_boost * weaken * def * res * vul * dmg_mit * broken)
    }

    fn talent(&self) -> f64 {
        let resistance_penalty_scale = vec![
            0.0, 10.00, 11.00, 12.00, 13.00, 14.00, 15.00, 16.25, 17.50, 18.75, 20.00, 21.00,
            22.00, 23.00, 24.00, 25.00,
        ];
        resistance_penalty_scale[self.character.skills.talent as usize]
    }

    fn crinsom_knot_bonus(&self) -> f64 {
        if self.character.traces.ability_3 {
            return 30.0 * std::cmp::min(3, self.thunder_core_bonus_stack) as f64;
        } else {
            return 0.0;
        }
    }

    fn the_abyss_multiplier(&self, teammates: &[Box<dyn Support>]) -> f64 {
        let mut num_same_path = 0;
        for teammate in teammates {
            if teammate.get_path() == self.character.path {
                num_same_path += 1;
            }
        }
        if self.character.eidolon >= 2 {
            num_same_path += 1;
        }
        num_same_path = std::cmp::min(2, num_same_path);
        let the_abyss = match self.character.traces.ability_2 {
            true => [1.0, 1.15, 1.6][num_same_path as usize],
            false => 1.0,
        };
        the_abyss
    }

    fn eidolon_1(&self) -> f64 {
        if self.character.eidolon >= 1 && self.activate_eidolon_1 {
            return 18.0;
        } else {
            return 0.0;
        }
    }

    fn eidolon_4(&self, enemy: &mut Enemy) {
        if self.character.eidolon >= 4 {
            enemy.vulnerability += 0.08;
        }
    }

    fn eidolon_6(&self) -> f64 {
        if self.character.eidolon >= 6 {
            return 20.0;
        } else {
            return 0.0;
        }
    }

    fn eidolon_6_skill_type(&self, skill_type: SkillType) -> SkillType {
        if self.character.eidolon >= 6 {
            return SkillType::Ultimate;
        }
        return skill_type;
    }

    fn crimson_knot(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Ultimate);
        let (mut base_stats, mut bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = [
            0.0, 0.0900, 0.0960, 0.1020, 0.1080, 0.1140, 0.1200, 0.1275, 0.1350, 0.1425, 0.1500,
            0.1560, 0.1620, 0.1680, 0.1740, 0.1800,
        ][self.character.skills.ult as usize];
        let ability_multiplier = match self.crimson_knot {
            1 | 2 => ability_multiplier + ability_multiplier * self.crimson_knot as f64,
            0 => ability_multiplier,
            _ => ability_multiplier + ability_multiplier * 3.0,
        };
        let mut enemy = enemy.clone();
        self.eidolon_4(&mut enemy);
        *bonus.entry(Stats::ResPenentration_).or_default() += self.talent() + self.eidolon_6();
        *bonus.entry(Stats::DmgBoost_).or_default() += self.crinsom_knot_bonus();
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            &enemy,
        )
    }

    fn rainblade(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Ultimate);
        let (mut base_stats, mut bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = [
            0.0, 0.1440, 0.1536, 0.1632, 0.1728, 0.1824, 0.1920, 0.2040, 0.2160, 0.2280, 0.2400,
            0.2496, 0.2592, 0.2688, 0.2784, 0.2880,
        ][self.character.skills.ult as usize];
        let mut enemy = enemy.clone();
        self.eidolon_4(&mut enemy);
        *bonus.entry(Stats::ResPenentration_).or_default() += self.talent() + self.eidolon_6();
        *bonus.entry(Stats::DmgBoost_).or_default() += self.crinsom_knot_bonus();
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            &enemy,
        )
    }

    fn stygian_resurge(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Ultimate);
        let (mut base_stats, mut bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = [
            0.0, 0.7200, 0.7680, 0.8160, 0.8640, 0.9120, 0.9600, 1.0200, 1.0800, 1.1400, 1.2000,
            1.2480, 1.2960, 1.3440, 1.3920, 1.4400,
        ][self.character.skills.ult as usize];
        let mut enemy = enemy.clone();
        self.eidolon_4(&mut enemy);
        *bonus.entry(Stats::ResPenentration_).or_default() += self.talent() + self.eidolon_6();
        *bonus.entry(Stats::DmgBoost_).or_default() += self.crinsom_knot_bonus();
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            &enemy,
        )
    }

    fn thunder_core(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Ultimate);
        let (mut base_stats, mut bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = if self.character.traces.ability_3 {
            0.25
        } else {
            0.0
        };
        let mut enemy = enemy.clone();
        self.eidolon_4(&mut enemy);
        *bonus.entry(Stats::ResPenentration_).or_default() += self.talent() + self.eidolon_6();
        *bonus.entry(Stats::DmgBoost_).or_default() += self.crinsom_knot_bonus();
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            &enemy,
        )
    }

    fn full_ultimate_multiplier_on_three_enemies(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let rainblade = self.rainblade(relics, enemy, teammates)?;
        let crinsom_knot = self.crimson_knot(relics, enemy, teammates)?;
        let bounce_atk = self.thunder_core(relics, enemy, teammates)?;
        let stygian_resurge = self.stygian_resurge(relics, enemy, teammates)?;
        Ok(rainblade * 3.0 + crinsom_knot * 9.0 + bounce_atk * 6.0 + stygian_resurge * 3.0)
    }

    fn full_ultimate_multiplier_on_single_enemy(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let rainblade = self.rainblade(relics, enemy, teammates)?;
        let crinsom_knot = self.crimson_knot(relics, enemy, teammates)?;
        let bounce_atk = self.thunder_core(relics, enemy, teammates)?;
        let stygian_resurge = self.stygian_resurge(relics, enemy, teammates)?;
        Ok(rainblade * 3.0 + crinsom_knot * 3.0 + bounce_atk * 6.0 + stygian_resurge)
    }

    fn skill_main_target(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Skill);
        let (mut base_stats, bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = [
            0.0, 0.8, 0.88, 0.96, 1.04, 1.12, 1.2, 1.3, 1.4, 1.5, 1.6, 1.68, 1.76, 1.84, 1.92, 2.0,
        ][self.character.skills.skill as usize];
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            enemy,
        )
    }

    fn skill_adjacent_target(
        &self,
        relics: &Relics,
        enemy: &Enemy,
        teammates: &[Box<dyn Support>],
    ) -> Result<f64> {
        let skill_type = self.eidolon_6_skill_type(SkillType::Skill);
        let (mut base_stats, bonus) = base_stats_and_bonus(
            &self.character,
            &self.light_cone,
            relics,
            &AttackType::Lightning,
            &skill_type,
            &DamageType::Normal,
            teammates,
        )?;
        let ability_multiplier = [
            0.0, 0.3, 0.33, 0.36, 0.39, 0.42, 0.45, 0.4875, 0.525, 0.5625, 0.6, 0.63, 0.66, 0.69,
            0.72, 0.75,
        ][self.character.skills.skill as usize];
        self.calculate_damage(
            teammates,
            ability_multiplier,
            &mut base_stats,
            &bonus,
            enemy,
        )
    }

    fn skill(&self, relics: &Relics, enemy: &Enemy, teammates: &[Box<dyn Support>]) -> Result<f64> {
        Ok(self.skill_adjacent_target(relics, enemy, teammates)? * 2.0
            + self.skill_main_target(relics, enemy, teammates)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        BaseStats, CharacterSkills, CharacterTraces, LightCone, LightConePassiveConfig, Path,
        RawRelic, RelicSetConfig, Slot, SubStats,
    };

    use super::*;

    #[test]
    fn test_ultimate_single() -> Result<()> {
        let (acheron, relics, enemy, teammates) = setup()?;
        let target = AcheronEvaluationTarget::UltimateSingle;

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &teammates)?,
            86053.53299468735
        );
        Ok(())
    }

    #[test]
    fn test_ultimate_aoe() -> Result<()> {
        let (acheron, relics, enemy, teammates) = setup()?;
        let target = AcheronEvaluationTarget::UltimateAoe;

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &teammates)?,
            183491.9791746075
        );
        Ok(())
    }

    #[test]
    fn test_skill() -> Result<()> {
        let (acheron, relics, enemy, teammates) = setup()?;
        let target = AcheronEvaluationTarget::Skill;

        assert_eq!(
            acheron.evaluate(&relics, &enemy, &target, &teammates)?,
            25324.79422299673
        );
        Ok(())
    }

    fn setup() -> Result<(Acheron, Relics, Enemy, Vec<Box<dyn Support>>)> {
        let character = Character {
            id: "1308".to_string(),
            name: "Acheron".to_string(),
            path: Path::Nihility,
            attack_type: AttackType::Lightning,
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
            base_hp: 1125.43,
            base_atk: 698.54,
            base_def: 436.59,
            base_spd: 101.00,
            base_aggro: 100,
            critical_chance: 5.0,
            critical_damage: 50.0,
            stat_bonus: BaseStats {
                atk_percentage: 28.0,
                crit_damage: 24.0,
                lightning_damage_boost: 8.0,
                ..Default::default()
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
            config: LightConePassiveConfig {
                stack_21001: 2,
                ..Default::default()
            },
        };
        let relics = Relics {
            relics: vec![
                RawRelic {
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
                }
                .try_into()?,
                RawRelic {
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
                }
                .try_into()?,
                RawRelic {
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
                }
                .try_into()?,
                RawRelic {
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
                }
                .try_into()?,
                RawRelic {
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
                }
                .try_into()?,
                RawRelic {
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
                }
                .try_into()?,
            ],
            config: RelicSetConfig {
                activate_102: true,
                activate_104: true,
                stack_105: 5,
                activate_107: true,
                activate_108: true,
                activate_109: true,
                activate_112_1: true,
                activate_112_2: true,
                stack_113: 5,
                stack_115: 5,
                stack_116: 5,
                activate_117_2pcs: true,
                stack_117: 5,
                activate_117_4pcs_extra: true,
                activate_120: true,
                activate_122: true,
                activate_123_1: true,
                activate_123_2: true,
                activate_125: true,
                activate_126: true,
                activate_305: true,
                stack_313: 5,
                stack_315: 5,
                activate_316: true,
                activate_318: true,
                stack_321: 4,
            },
        };
        let enemy = Enemy {
            level: 80,
            resistance: 0.0,
            dmg_mitigation: vec![],
            def_bonus: 0.0,
            vulnerability: 0.0,
            toughness_break: false,
            weaken: 0.0,
        };
        let acheron = Acheron {
            character,
            light_cone: Some(light_cone),
            crimson_knot: 9,
            crit: CritEnum::Avg,
            thunder_core_bonus_stack: 3,
            activate_eidolon_1: false,
        };
        Ok((
            acheron,
            relics,
            enemy,
            vec![Box::new(Pela {}), Box::new(Jiaoqiu {})],
        ))
    }

    struct Pela {}

    impl Support for Pela {
        fn get_path(&self) -> Path {
            Path::Nihility
        }
    }

    struct Jiaoqiu {}

    impl Support for Jiaoqiu {
        fn get_path(&self) -> Path {
            Path::Nihility
        }
    }
}
