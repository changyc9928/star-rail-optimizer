use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub path: Path,
    pub level: u8,
    pub ascension: u8,
    pub eidolon: u8,
    pub skills: CharacterSkills,
    pub traces: CharacterTraces,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterSkills {
    pub basic: u8,
    pub skill: u8,
    pub ult: u8,
    pub talent: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterTraces {
    pub ability_1: bool,
    pub ability_2: bool,
    pub ability_3: bool,
    pub stat_1: bool,
    pub stat_2: bool,
    pub stat_3: bool,
    pub stat_4: bool,
    pub stat_5: bool,
    pub stat_6: bool,
    pub stat_7: bool,
    pub stat_8: bool,
    pub stat_9: bool,
    pub stat_10: bool,
}

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct StatDetails {
//     pub hp: f64,
//     pub atk: f64,
//     pub def: f64,
//     pub spd: f64,
//     pub crit_rate: f64,
//     pub crit_dmg: f64,
//     pub break_effect: f64,
//     pub healing_bonus: f64,
//     pub burst_enery: f64,
//     pub energy_regeneration: f64,
//     pub effect_hit_rate: f64,
//     pub effect_res: f64,
//     pub dmg_bonus: f64,
//     pub resistance: f64,
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Path {
    #[serde(alias = "The Hunt", alias = "hunt")]
    TheHunt,
    #[serde(alias = "harmony")]
    Harmony,
    #[serde(alias = "preservation")]
    Preservation,
    #[serde(alias = "abundance")]
    Abundance,
    #[serde(alias = "nihility")]
    Nihility,
    #[serde(alias = "destruction")]
    Destruction,
    #[serde(alias = "erudition")]
    Erudition,
}

// impl StatDetails {
//     #[allow(clippy::too_many_arguments)]
//     pub async fn new(
//         base_atk: f64,
//         base_hp: f64,
//         base_def: f64,
//         base_spd: f64,
//         energy_cost: f64,
//         mut light_cone: Option<LightCone>,
//         relics: Relics,
//         traces_bonus: HashMap<Stats, f64>,
//         combat_type: Stats,
//     ) -> eyre::Result<Self> {
//         let set_bonus = relics.get_set_bonus(base_spd, &combat_type, traces_bonus.clone())?;
//         let hp = calculate_stat(
//             base_hp
//                 + if let Some(light_cone) = &mut light_cone {
//                     light_cone.get_main_stat().await?.hp
//                 } else {
//                     0.0
//                 },
//             Some(Stats::Hp),
//             Some(Stats::Hp_),
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let atk = calculate_stat(
//             base_atk
//                 + if let Some(light_cone) = &mut light_cone {
//                     light_cone.get_main_stat().await?.atk
//                 } else {
//                     0.0
//                 },
//             Some(Stats::Atk),
//             Some(Stats::Atk_),
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let def = calculate_stat(
//             base_def
//                 + if let Some(light_cone) = &mut light_cone {
//                     light_cone.get_main_stat().await?.def
//                 } else {
//                     0.0
//                 },
//             Some(Stats::Def),
//             Some(Stats::Def_),
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let spd = calculate_stat(
//             base_spd,
//             Some(Stats::Spd),
//             Some(Stats::Spd_),
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let crit_rate = calculate_stat(
//             5.0,
//             Some(Stats::CritRate_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let crit_dmg = calculate_stat(
//             50.0,
//             Some(Stats::CritDmg_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let break_effect = calculate_stat(
//             0.0,
//             Some(Stats::BreakEffect_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let healing_bonus = calculate_stat(
//             0.0,
//             Some(Stats::OutgoingHealingBoost_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let energy_regeneration = calculate_stat(
//             100.0,
//             Some(Stats::EnergyRegenerationRate_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let effect_hit_rate = calculate_stat(
//             0.0,
//             Some(Stats::EffectHitRate_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let effect_res = calculate_stat(
//             0.0,
//             Some(Stats::EffectRes_),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         let dmg_bonus = calculate_stat(
//             0.0,
//             Some(combat_type),
//             None,
//             &relics,
//             &set_bonus,
//             &traces_bonus,
//         );
//         Ok(Self {
//             hp,
//             atk,
//             def,
//             spd,
//             crit_rate,
//             crit_dmg,
//             break_effect,
//             healing_bonus,
//             burst_enery: energy_cost,
//             energy_regeneration,
//             effect_hit_rate,
//             effect_res,
//             dmg_bonus,
//             resistance: 0.0,
//         })
//     }
// }

// fn calculate_stat(
//     base_val: f64,
//     constant_stat: Option<Stats>,
//     percentage_stat: Option<Stats>,
//     relics: &Relics,
//     set_bonus: &HashMap<Stats, f64>,
//     traces_bonus: &HashMap<Stats, f64>,
// ) -> f64 {
//     let constant_val = if let Some(constant) = constant_stat {
//         relics.total.get(&constant).cloned().unwrap_or_default()
//             + set_bonus.get(&constant).cloned().unwrap_or_default()
//             + traces_bonus.get(&constant).cloned().unwrap_or_default()
//     } else {
//         0.0
//     };
//     let percentage_val = if let Some(percentage) = percentage_stat {
//         relics.total.get(&percentage).cloned().unwrap_or_default()
//             + set_bonus.get(&percentage).cloned().unwrap_or_default()
//             + traces_bonus.get(&percentage).cloned().unwrap_or_default()
//     } else {
//         0.0
//     };
//     base_val * (1.0 + percentage_val / 100.0) + constant_val
// }

// impl Character {
//     #[allow(dead_code, clippy::too_many_arguments)]
//     pub async fn new(
//         character_name: CharacterName,
//         path: Path,
//         lv: u8,
//         ascension: u8,
//         eidolon: u8,
//         skills: CharacterSkills,
//         traces: CharacterTraces,
//         light_cone: Option<LightCone>,
//         relics: Vec<Relic>,
//     ) -> eyre::Result<Self> {
//         let mut ret = Self {
//             name: character_name,
//             level: lv,
//             path,
//             ascension,
//             base_atk: Default::default(),
//             base_def: Default::default(),
//             base_hp: Default::default(),
//             base_spd: Default::default(),
//             light_cone: Default::default(),
//             relics: Default::default(),
//             eidolon,
//             skills,
//             traces,
//             stats_panel: Default::default(),
//             combat_type: Default::default(),
//         };
//         ret.add_base_stats().await?;
//         ret.update(relics, light_cone).await?;
//         Ok(ret)
//     }

//     pub async fn add_base_stats(&mut self) -> eyre::Result<()> {
//         let client = reqwest::Client::new();
//         let response = client
//             .get(format!(
//                 "https://api.yatta.top/hsr/v2/en/avatar/{}",
//                 self.name.get_yatta_id(&self.path)
//             ))
//             .send()
//             .await?
//             .text()
//             .await?;
//         let character_data: ProjectYattaCharacterQueryResponse = serde_json::from_str(&response)?;
//         let upgrade_data = character_data.data.upgrade;
//         let base_atk = upgrade_data[self.ascension as usize].skill_base.attack_base
//             + (self.level - 1) as f64 * upgrade_data[0].skill_add.attack_add;
//         let base_def = upgrade_data[self.ascension as usize]
//             .skill_base
//             .defence_base
//             + (self.level - 1) as f64 * upgrade_data[0].skill_add.defence_add;
//         let base_hp = upgrade_data[self.ascension as usize].skill_base.hp_base
//             + (self.level - 1) as f64 * upgrade_data[0].skill_add.hp_add;
//         let base_spd = upgrade_data[0].skill_base.speed_base;
//         self.base_atk = base_atk;
//         self.base_def = base_def;
//         self.base_hp = base_hp;
//         self.base_spd = base_spd;
//         self.combat_type = Some(CombatTypes::from(
//             character_data.data.types.combat_type.id.as_str(),
//         ));
//         self.stats_panel.burst_enery = character_data
//             .data
//             .traces
//             .main_skills
//             .iter()
//             .find(|s| match &s.1.skill_list {
//                 Some(s) => s.iter().any(|s| s.1.r#type == "Ultimate"),
//                 None => false,
//             })
//             .ok_or(eyre::eyre!("Missing ultimate"))?
//             .1
//             .skill_list
//             .as_ref()
//             .ok_or(eyre::eyre!("Missing ultimate"))?
//             .iter()
//             .next()
//             .ok_or(eyre::eyre!("Missing ultimate"))?
//             .1
//             .skill_points
//             .need
//             .ok_or(eyre::eyre!("Missing ultimate energy cost"))?
//             as f64;
//         let mut traces_bonus = HashMap::new();
//         let mut subskills: Vec<(String, ProjectYattaCharacterMainSkills)> =
//             character_data.data.traces.sub_skills.into_iter().collect();
//         subskills.sort_by(|x, y| x.0.cmp(&y.0));
//         let mut index = 0;
//         for (_, upgrades) in &subskills[3..] {
//             index += 1;
//             match index {
//                 1 if !self.traces.stat_1 => {
//                     continue;
//                 }
//                 2 if !self.traces.stat_2 => {
//                     continue;
//                 }
//                 3 if !self.traces.stat_3 => {
//                     continue;
//                 }
//                 4 if !self.traces.stat_4 => {
//                     continue;
//                 }
//                 5 if !self.traces.stat_5 => {
//                     continue;
//                 }
//                 6 if !self.traces.stat_6 => {
//                     continue;
//                 }
//                 7 if !self.traces.stat_7 => {
//                     continue;
//                 }
//                 8 if !self.traces.stat_8 => {
//                     continue;
//                 }
//                 9 if !self.traces.stat_9 => {
//                     continue;
//                 }
//                 10 if !self.traces.stat_10 => {
//                     continue;
//                 }
//                 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => (),
//                 _ => todo!(),
//             }
//             if let Some(name) = &upgrades.name {
//                 let key = match name.as_str() {
//                     "HP Boost" => Stats::Hp_,
//                     "CRIT Rate Boost" => Stats::CritRate_,
//                     "Effect RES Boost" => Stats::EffectRes_,
//                     key if key.starts_with("DMG Boost") => self
//                         .combat_type
//                         .clone()
//                         .ok_or(eyre::eyre!("Did not initialise the combat type"))?
//                         .into(),
//                     "DEF Boost" => Stats::Def_,
//                     "ATK Boost" => Stats::Atk_,
//                     "Effect Hit Rate Boost" => Stats::EffectHitRate_,
//                     "CRIT DMG Boost" => Stats::CritDmg_,
//                     "Break Enhance" => Stats::BreakEffect_,
//                     "SPD Boost" => Stats::Spd_,
//                     other => {
//                         if upgrades.point_type != "Attribute" {
//                             continue;
//                         } else {
//                             println!("{other}");
//                             todo!()
//                         }
//                     }
//                 };
//                 if let std::collections::hash_map::Entry::Vacant(e) =
//                     traces_bonus.entry(key.clone())
//                 {
//                     e.insert(
//                         upgrades
//                             .status_list
//                             .clone()
//                             .ok_or(eyre::eyre!("Missing status list"))?[0]
//                             .value
//                             * 100.0,
//                     );
//                 } else {
//                     let val = traces_bonus
//                         .get_mut(&key)
//                         .ok_or(eyre::eyre!("Missing key {key:?}"))?;
//                     *val += upgrades
//                         .status_list
//                         .clone()
//                         .ok_or(eyre::eyre!("Missing status list"))?[0]
//                         .value
//                         * 100.0;
//                 }
//             }
//         }
//         self.traces.total_bonus = traces_bonus;
//         Ok(())
//     }

//     pub async fn update(
//         &mut self,
//         relics: Vec<Relic>,
//         mut light_cone: Option<LightCone>,
//     ) -> eyre::Result<()> {
//         let (mut head, mut hands, mut body, mut feet, mut planar_sphere, mut link_rope) =
//             (None, None, None, None, None, None);
//         for relic in &relics {
//             match relic.slot {
//                 Slot::Head => head = Some(relic.clone()),
//                 Slot::Feet => feet = Some(relic.clone()),
//                 Slot::Body => body = Some(relic.clone()),
//                 Slot::Hands => hands = Some(relic.clone()),
//                 Slot::LinkRope => link_rope = Some(relic.clone()),
//                 Slot::PlanarSphere => planar_sphere = Some(relic.clone()),
//                 Slot::Dummy => eyre::bail!("Should not have dummy relics here"),
//             }
//         }

//         let relics = Relics::new(head, hands, body, feet, planar_sphere, link_rope);
//         if let Some(ref mut light_cone) = light_cone {
//             light_cone.get_main_stat().await?;
//         }
//         self.light_cone = light_cone.clone();
//         self.relics = relics.clone();
//         self.stats_panel = StatDetails::new(
//             self.base_atk,
//             self.base_hp,
//             self.base_def,
//             self.base_spd,
//             self.stats_panel.burst_enery,
//             light_cone,
//             relics,
//             self.traces.total_bonus.clone(),
//             self.combat_type
//                 .clone()
//                 .ok_or_else(|| eyre::eyre!("Missing combat type for {:?}", self.name))?
//                 .into(),
//         )
//         .await?;
//         Ok(())
//     }
// }
