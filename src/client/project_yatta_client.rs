use eyre::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub struct ProjectYattaClient {
    pub url: String,
}

#[derive(Deserialize)]
pub struct ProjectYattaCharacterResponse {
    pub data: CharacterData,
}

#[derive(Deserialize)]
pub struct CharacterData {
    pub traces: Traces,
    pub upgrade: Vec<CharacterUpgrade>,
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
    pub skill_base: CharacterSkillBase,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillAdd {
    pub attack_add: f64,
    pub defence_add: f64,
    pub h_p_add: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSkillBase {
    pub attack_base: f64,
    pub base_aggro: u64,
    pub critical_chance: f64,
    pub critical_damage: f64,
    pub defence_base: f64,
    pub h_p_base: f64,
    pub speed_base: f64,
}

#[derive(Deserialize)]
pub struct ProjectYattaLightConeResponse {
    pub data: LightConeData,
}

#[derive(Deserialize)]
pub struct LightConeData {
    pub upgrade: Vec<LightConeUpgrade>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightConeUpgrade {
    pub skill_add: SkillAdd,
    pub skill_base: LightConeSkillBase,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightConeSkillBase {
    pub attack_base: f64,
    pub h_p_base: f64,
    pub defence_base: f64,
}

impl ProjectYattaClient {
    pub async fn fetch_character_data(&self, id: &str) -> Result<ProjectYattaCharacterResponse> {
        let response = Client::new()
            .get(format!("{}avatar/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn fetch_light_cone_data(&self, id: &str) -> Result<ProjectYattaLightConeResponse> {
        let response = Client::new()
            .get(format!("{}equipment/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }
}
