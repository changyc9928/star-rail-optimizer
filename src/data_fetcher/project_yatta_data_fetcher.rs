use super::DataFetcher;
use crate::{
    client::project_yatta_client::{
        CharacterUpgrade, LightConeUpgrade, ProjectYattaClient, Traces,
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

    fn calculate_base_stats<T>(
        &self,
        upgrade: &T,
        level: u8,
        stat_getters: (
            fn(&T) -> f64,
            fn(&T) -> f64,
            fn(&T) -> f64,
            fn(&T) -> f64,
            fn(&T) -> f64,
            fn(&T) -> f64,
        ),
    ) -> (f64, f64, f64) {
        let (get_base_hp, get_add_hp, get_base_atk, get_add_atk, get_base_def, get_add_def) = (
            stat_getters.0,
            stat_getters.1,
            stat_getters.2,
            stat_getters.3,
            stat_getters.4,
            stat_getters.5,
        );

        let hp = get_base_hp(upgrade) + get_add_hp(upgrade) * (level - 1) as f64;
        let atk = get_base_atk(upgrade) + get_add_atk(upgrade) * (level - 1) as f64;
        let def = get_base_def(upgrade) + get_add_def(upgrade) * (level - 1) as f64;

        (hp, atk, def)
    }

    fn calculate_character_base_stats(
        &self,
        character: &Character,
        upgrades: &[CharacterUpgrade],
    ) -> CharacterEntity {
        let upgrade = &upgrades[character.ascension as usize];
        let (hp, atk, def) = self.calculate_base_stats(
            upgrade,
            character.level,
            (
                |u| u.skill_base.h_p_base,     // Base HP
                |u| u.skill_add.h_p_add,       // Add HP
                |u| u.skill_base.attack_base,  // Base ATK
                |u| u.skill_add.attack_add,    // Add ATK
                |u| u.skill_base.defence_base, // Base DEF
                |u| u.skill_add.defence_add,   // Add DEF
            ),
        );

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
        let (hp, atk, def) = self.calculate_base_stats(
            upgrade,
            light_cone.level,
            (
                |u| u.skill_base.h_p_base,     // Base HP
                |u| u.skill_add.h_p_add,       // Add HP
                |u| u.skill_base.attack_base,  // Base ATK
                |u| u.skill_add.attack_add,    // Add ATK
                |u| u.skill_base.defence_base, // Base DEF
                |u| u.skill_add.defence_add,   // Add DEF
            ),
        );

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
