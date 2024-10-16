use std::collections::HashMap;

use crate::{
    client::hoyowiki_client::HoyowikiClient,
    domain::{Character, CharacterEntity, LightCone, LightConeEntity, Path, Stats},
    engine::evaluator::StatBonusMap,
};
use eyre::{bail, eyre, Result};
use regex::Regex;
use serde::Deserialize;

use super::DataFetcher;

#[derive(Deserialize, Clone)]
pub struct Traces {
    name: Path,
    points: HashMap<String, Trace>,
}

#[derive(Deserialize, Clone)]
pub struct Trace {
    desc: String,
    title: String,
}

#[derive(Deserialize, Clone)]
pub struct Ascensions {
    list: Vec<Ascension>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ascension {
    key: String,
    combat_list: Vec<CombatList>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CombatList {
    key: String,
    values: Vec<String>,
}

#[deprecated]
pub struct HoyowikiDataFetcherService {
    pub client: HoyowikiClient,
}

#[allow(deprecated)]
impl DataFetcher for HoyowikiDataFetcherService {
    async fn fetch_character_data(&self, character: &Character) -> Result<CharacterEntity> {
        let components: Vec<Ascensions> = self.client.fetch_data("Ascend", &character.id).await?;
        let ascensions = components
            .first()
            .ok_or_else(|| eyre!("Ascension data not found"))?;
        let stat_bonus = self.calculate_trace_bonus(character).await?;
        match character.ascension {
            0 => self.calculate_character_base_stats(
                "Lv. 1",
                "Lv. 20",
                ascensions,
                character,
                &stat_bonus,
            ),
            1 => self.calculate_character_base_stats(
                "Lv. 20",
                "Lv. 30",
                ascensions,
                character,
                &stat_bonus,
            ),
            2 => self.calculate_character_base_stats(
                "Lv. 30",
                "Lv. 40",
                ascensions,
                character,
                &stat_bonus,
            ),
            3 => self.calculate_character_base_stats(
                "Lv. 40",
                "Lv. 50",
                ascensions,
                character,
                &stat_bonus,
            ),
            4 => self.calculate_character_base_stats(
                "Lv. 50",
                "Lv. 60",
                ascensions,
                character,
                &stat_bonus,
            ),
            5 => self.calculate_character_base_stats(
                "Lv. 60",
                "Lv. 70",
                ascensions,
                character,
                &stat_bonus,
            ),
            6 => self.calculate_character_base_stats(
                "Lv. 70",
                "Lv. 80",
                ascensions,
                character,
                &stat_bonus,
            ),
            _ => Err(eyre!("Invalid ascension value")),
        }
    }

    async fn fetch_light_cone_data(&self, light_cone: &LightCone) -> Result<LightConeEntity> {
        let components: Vec<Ascensions> = self.client.fetch_data("Ascend", &light_cone.id).await?;
        let ascensions = components
            .first()
            .ok_or_else(|| eyre!("Ascension data not found"))?;
        match light_cone.ascension {
            0 => self.calculate_light_cone_base_stats("Lv. 1", "Lv. 20", ascensions, light_cone),
            1 => self.calculate_light_cone_base_stats("Lv. 20", "Lv. 30", ascensions, light_cone),
            2 => self.calculate_light_cone_base_stats("Lv. 30", "Lv. 40", ascensions, light_cone),
            3 => self.calculate_light_cone_base_stats("Lv. 40", "Lv. 50", ascensions, light_cone),
            4 => self.calculate_light_cone_base_stats("Lv. 50", "Lv. 60", ascensions, light_cone),
            5 => self.calculate_light_cone_base_stats("Lv. 60", "Lv. 70", ascensions, light_cone),
            6 => self.calculate_light_cone_base_stats("Lv. 70", "Lv. 80", ascensions, light_cone),
            _ => Err(eyre!("Invalid ascension value")),
        }
    }
}

#[allow(deprecated)]
impl HoyowikiDataFetcherService {
    fn extract_ascension_value(&self, stat: &str, ascension: &Ascension) -> Result<Vec<u64>> {
        let term = stat.split(" ").collect::<Vec<_>>();
        let specific_re = Regex::new(&format!(r"{}\s*{}", term[0], term[1])).unwrap();
        ascension
            .combat_list
            .iter()
            .find(|c| specific_re.is_match(&c.key))
            .ok_or_else(|| eyre!("{stat} not found"))?
            .values
            .iter()
            .map(|v| {
                if v == "-" {
                    return Ok(u64::MAX);
                }
                Ok(v.parse()?)
            })
            .collect::<Result<Vec<_>>>()
    }

    fn extract_trace_bonus(&self, key: &str, traces: &Traces) -> Result<(Stats, f64)> {
        let title_mapper = |title: &str| match title {
            "HP Boost" => Stats::Hp_,
            "ATK Boost" => Stats::Atk_,
            "DEF Boost" => Stats::Def_,
            "SPD Boost" => Stats::Spd_,
            "CRIT Rate Boost" => Stats::CritRate_,
            "CRIT DMG Boost" => Stats::CritDmg_,
            "Effect RES Boost" => Stats::EffectRes_,
            "Beak Effect Boost" => Stats::BreakEffect_,
            "Energy Regeneration Boost" => Stats::EnergyRegenerationRate_,
            "Effect Hit Rate Boost" => Stats::EffectHitRate_,
            _ => todo!(),
        };
        let extract_boost = |desc: &str| -> Result<f64> {
            Regex::new(r"(\d+(\.\d+)?)%")
                .map(|re| {
                    re.captures(desc)
                        .and_then(|captures| captures.get(1))
                        .ok_or_else(|| eyre!("No boost value found"))
                        .and_then(|value| {
                            value
                                .as_str()
                                .parse::<f64>()
                                .map_err(|e| eyre!("Failed to parse boost value: {}", e))
                        })
                })
                .map_err(|e| eyre!("Regex compilation failed: {}", e))?
        };
        traces
            .points
            .iter()
            .find(|(this_key, _)| *this_key == key)
            .map(|(_, value)| Ok((title_mapper(&value.title), extract_boost(&value.desc)?)))
            .ok_or_else(|| eyre!("Key {key} not found"))?
    }

    fn calculate_character_base_stats(
        &self,
        lo: &str,
        hi: &str,
        ascensions: &Ascensions,
        character: &Character,
        stat_bonus: &StatBonusMap,
    ) -> Result<CharacterEntity> {
        let lower_bound = ascensions
            .list
            .iter()
            .find(|a| a.key == lo)
            .ok_or_else(|| eyre!("Base value {lo} not found"))?;
        let upper_bound = ascensions
            .list
            .iter()
            .find(|a| a.key == hi)
            .ok_or_else(|| eyre!("Base value {hi} not found"))?;
        let calc_gradient = |hi: u64, lo: u64, lo_key: &str| -> f64 {
            if lo_key == "Lv. 1" {
                (hi - lo) as f64 / 19.0
            } else {
                (hi - lo) as f64 / 10.0
            }
        };
        let extract_level = |lo_key: &str| -> Result<u8> {
            let re = Regex::new(r"Lv\. (\d+)").unwrap();
            if let Some(caps) = re.captures(lo_key) {
                Ok(caps[1].parse::<u8>()?)
            } else {
                bail!("Invalid level")
            }
        };
        let calc_base_stat = |lo: Vec<u64>, hi: Vec<u64>, lo_key: &str| -> Result<f64> {
            Ok(calc_gradient(hi[0], lo[1], lo_key)
                * (character.level - extract_level(lo_key)?) as f64
                + lo[1] as f64)
        };
        let (hp_lo, hp_hi) = (
            self.extract_ascension_value("Base HP", lower_bound)?,
            self.extract_ascension_value("Base HP", upper_bound)?,
        );
        let (atk_lo, atk_hi) = (
            self.extract_ascension_value("Base ATK", lower_bound)?,
            self.extract_ascension_value("Base ATK", upper_bound)?,
        );
        let (def_lo, def_hi) = (
            self.extract_ascension_value("Base DEF", lower_bound)?,
            self.extract_ascension_value("Base DEF", upper_bound)?,
        );
        let (spd_lo, spd_hi) = (
            self.extract_ascension_value("Base SPD", lower_bound)?,
            self.extract_ascension_value("Base SPD", upper_bound)?,
        );
        Ok(CharacterEntity {
            base_hp: calc_base_stat(hp_lo, hp_hi, lo)?,
            base_atk: calc_base_stat(atk_lo, atk_hi, lo)?,
            base_def: calc_base_stat(def_lo, def_hi, lo)?,
            base_spd: calc_base_stat(spd_lo, spd_hi, lo)?,
            _base_aggro: 0,
            critical_chance: 5.0,
            critical_damage: 50.0,
            stat_bonus: stat_bonus.clone(),
            _character: character.clone(),
        })
    }

    fn calculate_light_cone_base_stats(
        &self,
        lo: &str,
        hi: &str,
        ascensions: &Ascensions,
        light_cone: &LightCone,
    ) -> Result<LightConeEntity> {
        let lower_bound = ascensions
            .list
            .iter()
            .find(|a| a.key == lo)
            .ok_or_else(|| eyre!("Base value {lo} not found"))?;
        let upper_bound = ascensions
            .list
            .iter()
            .find(|a| a.key == hi)
            .ok_or_else(|| eyre!("Base value {hi} not found"))?;
        let calc_gradient = |hi: u64, lo: u64, lo_key: &str| -> f64 {
            if lo_key == "Lv. 1" {
                (hi - lo) as f64 / 19.0
            } else {
                (hi - lo) as f64 / 10.0
            }
        };
        let extract_level = |lo_key: &str| -> Result<u8> {
            let re = Regex::new(r"Lv\. (\d+)").unwrap();
            if let Some(caps) = re.captures(lo_key) {
                Ok(caps[1].parse::<u8>()?)
            } else {
                bail!("Invalid level")
            }
        };
        let calc_base_stat = |lo: Vec<u64>, hi: Vec<u64>, lo_key: &str| -> Result<f64> {
            Ok(calc_gradient(hi[0], lo[1], lo_key)
                * (light_cone.level - extract_level(lo_key)?) as f64
                + lo[1] as f64)
        };
        let (hp_lo, hp_hi) = (
            self.extract_ascension_value("Base HP", lower_bound)?,
            self.extract_ascension_value("Base HP", upper_bound)?,
        );
        let (atk_lo, atk_hi) = (
            self.extract_ascension_value("Base ATK", lower_bound)?,
            self.extract_ascension_value("Base ATK", upper_bound)?,
        );
        let (def_lo, def_hi) = (
            self.extract_ascension_value("Base DEF", lower_bound)?,
            self.extract_ascension_value("Base DEF", upper_bound)?,
        );
        Ok(LightConeEntity {
            base_hp: calc_base_stat(hp_lo, hp_hi, lo)?,
            base_atk: calc_base_stat(atk_lo, atk_hi, lo)?,
            base_def: calc_base_stat(def_lo, def_hi, lo)?,
            _light_cone: light_cone.clone(),
        })
    }

    fn trace_mapping(&self, path: &Path, key: &str) -> Result<String> {
        Ok(match path {
            Path::Destruction => match key {
                "stat_1" => "D1",
                "stat_2" => "D7",
                "stat_3" => "D8",
                "stat_4" => "D9",
                "stat_5" => "D3",
                "stat_6" => "D4",
                "stat_7" => "D5",
                "stat_8" => "B2",
                "stat_9" => "B4",
                "stat_10" => "B3",
                _ => return Err(eyre!("Invalid trace key")),
            },
            Path::Preservation => match key {
                "stat_1" => "D1",
                "stat_2" => "D6",
                "stat_3" => "D7",
                "stat_4" => "E1",
                "stat_5" => "D3",
                "stat_6" => "D4",
                "stat_7" => "C1",
                "stat_8" => "B2",
                "stat_9" => "B4",
                "stat_10" => "B3",
                _ => return Err(eyre!("Invalid trace key")),
            },
            _ => todo!(),
        }
        .to_string())
    }

    async fn calculate_trace_bonus(&self, character: &Character) -> Result<StatBonusMap> {
        let components: Vec<Traces> = self.client.fetch_data("Traces", &character.id).await?;
        let traces = components
            .first()
            .ok_or_else(|| eyre!("Traces data not found"))?;
        let mut stat_bonus = HashMap::new();
        let mut trace_bonus_adder = |key: &str| -> Result<()> {
            let (stat, bonus) =
                self.extract_trace_bonus(&self.trace_mapping(&traces.name, key)?, traces)?;
            *stat_bonus.entry(stat).or_default() += bonus;
            Ok(())
        };
        if character.traces.stat_1 {
            trace_bonus_adder("stat_1")?
        }
        if character.traces.stat_2 {
            trace_bonus_adder("stat_2")?
        }
        if character.traces.stat_3 {
            trace_bonus_adder("stat_3")?
        }
        if character.traces.stat_4 {
            trace_bonus_adder("stat_4")?
        }
        if character.traces.stat_5 {
            trace_bonus_adder("stat_5")?
        }
        if character.traces.stat_6 {
            trace_bonus_adder("stat_6")?
        }
        if character.traces.stat_7 {
            trace_bonus_adder("stat_7")?
        }
        if character.traces.stat_8 {
            trace_bonus_adder("stat_8")?
        }
        if character.traces.stat_9 {
            trace_bonus_adder("stat_9")?
        }
        if character.traces.stat_10 {
            trace_bonus_adder("stat_10")?
        }
        Ok(stat_bonus)
    }
}
