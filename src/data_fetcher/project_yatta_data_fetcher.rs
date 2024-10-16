use super::DataFetcher;
use crate::{
    client::project_yatta_client::{
        CharacterUpgrade, LightConeUpgrade, ProjectYattaClient, Traces, Upgrade,
    },
    domain::{Character, CharacterEntity, LightCone, LightConeEntity, Stats},
    engine::evaluator::StatBonusMap,
    utils::trace_title_mapper::title_mapper,
};
use eyre::{eyre, Result};
use std::collections::HashMap;

pub struct ProjectYattaDataFetcher {
    pub client: ProjectYattaClient,
}

impl ProjectYattaDataFetcher {
    fn calculate_trace_bonus(&self, character: &Character, trace: &Traces) -> Result<StatBonusMap> {
        let mut bonus: HashMap<Stats, f64> = HashMap::new();

        for i in 1..=10 {
            let id = format!("{:03}", 200 + i);
            if character.traces.get_stat(i) {
                self.insert_trace_bonus(&mut bonus, trace, &character.id, &id)?;
            }
        }

        Ok(bonus)
    }

    fn insert_trace_bonus(
        &self,
        bonus: &mut HashMap<Stats, f64>,
        trace: &Traces,
        character_id: &str,
        id: &str,
    ) -> Result<()> {
        let subskill = trace
            .sub_skills
            .get(&format!("{}{}", character_id, id))
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
        character: &Character,
        upgrades: &[CharacterUpgrade],
    ) -> CharacterEntity {
        let upgrade = &upgrades[character.ascension as usize];
        let (hp, atk, def) = self.calculate_base_stats(upgrade, character.level);

        CharacterEntity {
            base_hp: hp,
            base_atk: atk,
            base_def: def,
            base_spd: upgrade.skill_base.speed_base.unwrap_or(0.0), // Handle optional
            _base_aggro: upgrade.skill_base.base_aggro.unwrap_or(0), // Handle optional
            critical_chance: upgrade.skill_base.critical_chance.unwrap_or(0.0) * 100.0,
            critical_damage: upgrade.skill_base.critical_damage.unwrap_or(0.0) * 100.0,
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
        let (hp, atk, def) = self.calculate_base_stats(upgrade, light_cone.level);

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
