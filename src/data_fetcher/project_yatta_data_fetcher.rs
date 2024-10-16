use super::DataFetcher;
use crate::{
    client::project_yatta_client::{
        CharacterUpgrade, LightConeUpgrade, ProjectYattaClient, Traces,
    },
    domain::{Character, CharacterEntity, LightCone, LightConeEntity, Stats},
    engine::evaluator::StatBonusMap,
};
use eyre::{eyre, Result};
use std::collections::HashMap;

pub struct ProjectYattaDataFetcher {
    pub client: ProjectYattaClient,
}

impl ProjectYattaDataFetcher {
    fn calculate_trace_bonus(&self, character: &Character, trace: &Traces) -> Result<StatBonusMap> {
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
        let mut bonus: HashMap<Stats, f64> = HashMap::new();
        let mut inserter = |id: &str| -> Result<()> {
            let subskill = trace
                .sub_skills
                .get(&format!("{}{}", character.id, id))
                .ok_or_else(|| eyre!("Trace not found"))?;
            *bonus.entry(title_mapper(&subskill.name)).or_default() += subskill
                .status_list
                .clone()
                .ok_or_else(|| eyre!("Missing trace info"))?
                .first()
                .ok_or_else(|| eyre!("No entry in subskill status list"))?
                .value
                * 100.0;
            Ok(())
        };
        if character.traces.stat_1 {
            inserter("201")?
        }
        if character.traces.stat_2 {
            inserter("202")?
        }
        if character.traces.stat_3 {
            inserter("203")?
        }
        if character.traces.stat_4 {
            inserter("204")?
        }
        if character.traces.stat_5 {
            inserter("205")?
        }
        if character.traces.stat_6 {
            inserter("206")?
        }
        if character.traces.stat_7 {
            inserter("207")?
        }
        if character.traces.stat_8 {
            inserter("208")?
        }
        if character.traces.stat_9 {
            inserter("209")?
        }
        if character.traces.stat_10 {
            inserter("210")?
        }
        Ok(bonus)
    }

    fn calculate_character_base_stats(
        &self,
        character: &Character,
        upgrades: &[CharacterUpgrade],
    ) -> CharacterEntity {
        let upgrade = &upgrades[character.ascension as usize];
        let hp =
            upgrade.skill_base.h_p_base + upgrade.skill_add.h_p_add * (character.level - 1) as f64;
        let atk = upgrade.skill_base.attack_base
            + upgrade.skill_add.attack_add * (character.level - 1) as f64;
        let def = upgrade.skill_base.defence_base
            + upgrade.skill_add.defence_add * (character.level - 1) as f64;
        CharacterEntity {
            base_hp: hp,
            base_atk: atk,
            base_def: def,
            base_spd: upgrade.skill_base.speed_base,
            _base_aggro: upgrade.skill_base.base_aggro,
            critical_chance: upgrade.skill_base.critical_chance * 100.0,
            critical_damage: upgrade.skill_base.critical_damage * 100.0,
            stat_bonus: HashMap::new(),
            _character: character.clone(),
        }
    }

    fn calculate_light_cone_base_stats(
        &self,
        light_cone: &LightCone,
        upgrades: &[LightConeUpgrade],
    ) -> LightConeEntity {
        let upgrade = &upgrades[light_cone.ascension as usize];
        let hp =
            upgrade.skill_base.h_p_base + upgrade.skill_add.h_p_add * (light_cone.level - 1) as f64;
        let atk = upgrade.skill_base.attack_base
            + upgrade.skill_add.attack_add * (light_cone.level - 1) as f64;
        let def = upgrade.skill_base.defence_base
            + upgrade.skill_add.defence_add * (light_cone.level - 1) as f64;
        LightConeEntity {
            base_hp: hp,
            base_atk: atk,
            base_def: def,
            _light_cone: light_cone.clone(),
        }
    }
}

impl DataFetcher for ProjectYattaDataFetcher {
    async fn fetch_character_data(&self, character: &Character) -> Result<CharacterEntity> {
        let response = self.client.fetch_character_data(&character.id).await?;
        let mut character_entity =
            self.calculate_character_base_stats(character, &response.data.upgrade);
        character_entity.stat_bonus =
            self.calculate_trace_bonus(character, &response.data.traces)?;
        Ok(character_entity)
    }

    async fn fetch_light_cone_data(&self, light_cone: &LightCone) -> Result<LightConeEntity> {
        let response = self.client.fetch_light_cone_data(&light_cone.id).await?;
        Ok(self.calculate_light_cone_base_stats(light_cone, &response.data.upgrade))
    }
}
