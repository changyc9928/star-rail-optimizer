use eyre::{eyre, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub trait Upgrade {
    fn skill_base(&self) -> &SkillBase;
    fn skill_add(&self) -> &SkillAdd;
}

impl Upgrade for CharacterUpgrade {
    fn skill_base(&self) -> &SkillBase {
        &self.skill_base
    }

    fn skill_add(&self) -> &SkillAdd {
        &self.skill_add
    }
}

impl Upgrade for LightConeUpgrade {
    fn skill_base(&self) -> &SkillBase {
        &self.skill_base
    }

    fn skill_add(&self) -> &SkillAdd {
        &self.skill_add
    }
}

pub struct ProjectYattaClient {
    pub url: String,
    pub light_cone_cache: HashMap<String, ProjectYattaLightConeResponse>,
}

#[derive(Deserialize)]
pub struct ProjectYattaCharacterResponse {
    pub data: CharacterData,
}

#[derive(Deserialize)]
pub struct CharacterData {
    pub traces: Traces,
    pub upgrade: Vec<CharacterUpgrade>,
    pub types: Types,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Types {
    pub combat_type: CombatType,
}

#[derive(Deserialize)]
pub struct CombatType {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traces {
    pub sub_skills: HashMap<String, SubSkill>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubSkill {
    pub name: String,
    pub status_list: Option<Vec<StatusList>>,
}

#[derive(Deserialize, Clone)]
pub struct StatusList {
    pub value: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterUpgrade {
    pub skill_add: SkillAdd,
    pub skill_base: SkillBase,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillAdd {
    pub attack_add: f64,
    pub defence_add: f64,
    pub h_p_add: f64,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillBase {
    pub attack_base: f64,
    pub defence_base: f64,
    pub h_p_base: f64,
    pub speed_base: Option<f64>, // Optional, since only Character has speed_base
    pub base_aggro: Option<u64>, // Optional, since only Character has base_aggro
    pub critical_chance: Option<f64>, // Optional, since only Character has critical stats
    pub critical_damage: Option<f64>, // Optional, since only Character has critical stats
}

#[derive(Deserialize, Clone)]
pub struct ProjectYattaLightConeResponse {
    pub data: LightConeData,
}

#[derive(Deserialize, Clone)]
pub struct LightConeData {
    pub upgrade: Vec<LightConeUpgrade>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightConeUpgrade {
    pub skill_add: SkillAdd,
    pub skill_base: SkillBase,
}

impl ProjectYattaClient {
    pub async fn fetch_character_data(&self, id: &str) -> Result<ProjectYattaCharacterResponse> {
        Ok(Client::new()
            .get(format!("{}avatar/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn fetch_light_cone_data(
        &mut self,
        id: &str,
    ) -> Result<ProjectYattaLightConeResponse> {
        if self.light_cone_cache.contains_key(id) {
            return self
                .light_cone_cache
                .get(id)
                .cloned()
                .ok_or_else(|| eyre!("Unexpected error"));
        }
        let response: ProjectYattaLightConeResponse = Client::new()
            .get(format!("{}equipment/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        self.light_cone_cache
            .insert(id.to_owned(), response.clone());
        Ok(response)
    }
}
