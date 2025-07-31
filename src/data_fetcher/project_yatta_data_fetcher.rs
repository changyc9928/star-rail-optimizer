use super::DataFetcher;
use crate::{
    client::project_yatta_client::{
        CharacterUpgrade, LightConeUpgrade, ProjectYattaClient, Traces, Upgrade,
    },
    domain::{
        BaseStats, Character, LightCone, LightConeEntity, LightConePassiveConfig, RawCharacter,
    },
};
use async_trait::async_trait;
use eyre::{eyre, Result};

pub struct ProjectYattaDataFetcher {
    pub client: ProjectYattaClient,
}

impl ProjectYattaDataFetcher {
    fn calculate_trace_bonus(&self, character: &RawCharacter, trace: &Traces) -> Result<BaseStats> {
        let mut base_stats = BaseStats::default();

        for i in 1..=10 {
            let id = format!("{:03}", 200 + i);
            if character.traces.get_stat(i) {
                self.insert_trace_bonus(&mut base_stats, trace, &character.id, &id)?;
            }
        }

        Ok(base_stats)
    }

    fn insert_trace_bonus(
        &self,
        base_stats: &mut BaseStats,
        trace: &Traces,
        character_id: &str,
        id: &str,
    ) -> Result<()> {
        let subskill = trace
            .sub_skills
            .get(&format!("{}{}", character_id, id))
            .ok_or_else(|| eyre!("Trace not found"))?;

        let extract = || {
            Ok::<f64, eyre::Report>(
                subskill
                    .status_list
                    .clone()
                    .ok_or_else(|| eyre!("Missing trace info"))?
                    .first()
                    .ok_or_else(|| eyre!("No entry in subskill status list"))?
                    .value
                    * 100.0,
            )
        };

        match subskill.name.as_str() {
            "HP Boost" => base_stats.hp_percentage += extract()?,
            "ATK Boost" => base_stats.atk_percentage += extract()?,
            "DEF Boost" => base_stats.def_percentage += extract()?,
            "SPD Boost" => base_stats.spd_percentage += extract()?,
            "CRIT Rate Boost" => base_stats.crit_rate += extract()?,
            "CRIT DMG Boost" => base_stats.crit_damage += extract()?,
            "Effect RES Boost" => base_stats.effect_resistance += extract()?,
            "Break Boost" | "Break Enhance" => base_stats.break_effect += extract()?,
            "Energy Regeneration Boost" => base_stats.energy_regeneration_rate += extract()?,
            "Effect Hit Rate Boost" => base_stats.effect_hit_rate += extract()?,
            "DMG Boost" => {
                base_stats.fire_damage_boost += extract()?;
                base_stats.ice_damage_boost += extract()?;
                base_stats.wind_damage_boost += extract()?;
                base_stats.lightning_damage_boost += extract()?;
                base_stats.physical_damage_boost += extract()?;
                base_stats.quantum_damage_boost += extract()?;
                base_stats.imaginary_damage_boost += extract()?;
            }
            "DMG Boost: Ice" => base_stats.ice_damage_boost += extract()?,
            "DMG Boost: Fire" | "DMG Boost Fire" => base_stats.fire_damage_boost += extract()?,
            "DMG Boost: Wind" => base_stats.wind_damage_boost += extract()?,
            "DMG Boost: Lightning" => base_stats.lightning_damage_boost += extract()?,
            "DMG Boost: Imaginary" => base_stats.imaginary_damage_boost += extract()?,
            "DMG Boost: Quantum" => base_stats.quantum_damage_boost += extract()?,
            "DMG Boost: Physical" => base_stats.physical_damage_boost += extract()?,
            _ => todo!(),
        };

        Ok(())
    }

    fn calculate_base_stats<T: Upgrade>(&self, upgrade: &T, level: u8) -> (f64, f64, f64) {
        let hp = upgrade.skill_base().h_p_base + upgrade.skill_add().h_p_add * (level - 1) as f64;
        let atk =
            upgrade.skill_base().attack_base + upgrade.skill_add().attack_add * (level - 1) as f64;
        let def = upgrade.skill_base().defence_base
            + upgrade.skill_add().defence_add * (level - 1) as f64;

        (hp, atk, def)
    }

    fn calculate_character_base_stats(
        &self,
        character: &RawCharacter,
        upgrades: &[CharacterUpgrade],
        combat_type: &str,
    ) -> Character {
        let upgrade = &upgrades[character.ascension as usize];
        let (hp, atk, def) = self.calculate_base_stats(upgrade, character.level);

        Character {
            base_hp: hp,
            base_atk: atk,
            base_def: def,
            base_spd: upgrade.skill_base.speed_base.unwrap_or(0.0), // Handle optional
            base_aggro: upgrade.skill_base.base_aggro.unwrap_or(0), // Handle optional
            critical_chance: upgrade.skill_base.critical_chance.unwrap_or(0.0) * 100.0,
            critical_damage: upgrade.skill_base.critical_damage.unwrap_or(0.0) * 100.0,
            stat_bonus: BaseStats::default(),
            id: character.id.clone(),
            name: character.name.clone(),
            path: character.path.clone(),
            attack_type: match combat_type {
                "Wind" => crate::domain::AttackType::Wind,
                "Fire" => crate::domain::AttackType::Fire,
                "Lightning" => crate::domain::AttackType::Lightning,
                "Ice" => crate::domain::AttackType::Ice,
                "Physical" => crate::domain::AttackType::Physical,
                "Quantum" => crate::domain::AttackType::Quantum,
                "Imaginary" => crate::domain::AttackType::Imaginary,
                _ => todo!(),
            },
            level: character.level,
            ascension: character.ascension,
            eidolon: character.eidolon,
            skills: character.skills.clone(),
            traces: character.traces.clone(),
        }
    }

    fn calculate_light_cone_base_stats(
        &self,
        light_cone: &LightCone,
        upgrades: &[LightConeUpgrade],
    ) -> LightConeEntity {
        let upgrade = &upgrades[light_cone.ascension as usize];
        let (hp, atk, def) = self.calculate_base_stats(upgrade, light_cone.level);

        LightConeEntity {
            base_hp: hp,
            base_atk: atk,
            base_def: def,
            _light_cone: light_cone.clone(),
            config: LightConePassiveConfig::default(),
        }
    }
}

#[async_trait]
impl DataFetcher for ProjectYattaDataFetcher {
    async fn fetch_character_data(&self, character: &RawCharacter) -> Result<Character> {
        let response = self.client.fetch_character_data(&character.id).await?;
        let mut character_entity = self.calculate_character_base_stats(
            character,
            &response.data.upgrade,
            &response.data.types.combat_type.id,
        );
        character_entity.stat_bonus =
            self.calculate_trace_bonus(character, &response.data.traces)?;
        Ok(character_entity)
    }

    async fn fetch_light_cone_data(&mut self, light_cone: &LightCone) -> Result<LightConeEntity> {
        let response = self.client.fetch_light_cone_data(&light_cone.id).await?;
        Ok(self.calculate_light_cone_base_stats(light_cone, &response.data.upgrade))
    }
}
