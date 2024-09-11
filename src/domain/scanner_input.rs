use std::collections::HashMap;

use super::{Character, CharacterName, LightCone, Relic, StatDetails, Stats};
use kdam::BarExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScannerInput {
    pub source: String,
    pub version: u8,
    pub light_cones: Vec<LightCone>,
    pub relics: Vec<Relic>,
    pub characters: Vec<Character>,
}

impl ScannerInput {
    pub async fn update(&mut self) -> eyre::Result<()> {
        for character in &mut self.characters {
            let light_cone = self
                .light_cones
                .iter()
                .find(|l| l.location == Some(character.key.clone()))
                .cloned();
            let relics = self
                .relics
                .iter()
                .filter(|r| r.location == Some(character.key.clone()))
                .cloned()
                .collect();
            character.add_base_stats().await?;
            character.update(relics, light_cone).await?;
        }
        Ok(())
    }

    /// Remove the duplication of stats, especially for relics with level 1
    fn deduplicate(
        &self,
        relics: Vec<Relic>,
        related_stats: Vec<Stats>,
    ) -> eyre::Result<Vec<Relic>> {
        let mut filter = HashMap::new();
        let hash_key_vec: Vec<(Stats, String)> = related_stats
            .iter()
            .filter_map(|s| {
                if related_stats.contains(s) {
                    Some((s.clone(), format!("{:?}: ", s)))
                } else {
                    None
                }
            })
            .collect();
        relics.iter().try_for_each(|r| -> eyre::Result<()> {
            let mut hash_key = HashMap::new();
            if related_stats.contains(&r.mainstat) {
                hash_key.insert(r.mainstat.clone(), r.get_mainstat());
            }
            r.substats.iter().try_for_each(|s| -> eyre::Result<()> {
                if related_stats.contains(&s.key) {
                    if !hash_key.contains_key(&s.key) {
                        hash_key.insert(s.key.clone(), s.value);
                    } else {
                        let val = hash_key
                            .get_mut(&s.key)
                            .ok_or(eyre::eyre!("Missing stats"))?;
                        *val += s.value;
                    }
                }
                Ok(())
            })?;
            // TODO: take into account the set bonus
            let key: Vec<String> = hash_key_vec
                .iter()
                .filter_map(|(k, v)| hash_key.get(k).map(|val| format!("{:?}{}", v, val)))
                .collect::<Vec<String>>();
            let join = key.join(", ");
            filter.insert(join, r.clone());
            Ok(())
        })?;
        Ok(filter.values().cloned().collect())
    }

    pub async fn optimize(
        &mut self,
        character: CharacterName,
        optimization_target: Stats,
        min_requirement: HashMap<Stats, f64>,
    ) -> eyre::Result<(Vec<Relic>, StatDetails)> {
        let mut heads = vec![];
        let mut hands = vec![];
        let mut bodies = vec![];
        let mut feet = vec![];
        let mut spheres = vec![];
        let mut ropes = vec![];

        let mut related_stats: Vec<Stats> = min_requirement.keys().cloned().collect();
        related_stats.push(optimization_target.clone());
        if optimization_target == Stats::Hp || min_requirement.contains_key(&Stats::Hp) {
            related_stats.push(Stats::Hp_);
        }
        if optimization_target == Stats::Atk || min_requirement.contains_key(&Stats::Atk) {
            related_stats.push(Stats::Atk_);
        }
        if optimization_target == Stats::Def || min_requirement.contains_key(&Stats::Def) {
            related_stats.push(Stats::Def_);
        }
        if optimization_target == Stats::Spd || min_requirement.contains_key(&Stats::Spd) {
            related_stats.push(Stats::Spd_);
        }

        self.relics.iter().for_each(|r| {
            if r.rarity == 5
            // && (related_stats.contains(&r.mainstat)
            //     || related_stats
            //         .iter()
            //         .map(|s| {
            //             r.substats
            //                 .iter()
            //                 .map(|s| s.key.clone())
            //                 .collect::<Vec<Stats>>()
            //                 .contains(s)
            //         })
            //         .collect::<Vec<bool>>()
            //         .contains(&true))
            {
                match r.slot {
                    crate::domain::Slot::Head => heads.push(r.clone()),
                    crate::domain::Slot::Feet => feet.push(r.clone()),
                    crate::domain::Slot::Body => bodies.push(r.clone()),
                    crate::domain::Slot::Hands => hands.push(r.clone()),
                    crate::domain::Slot::LinkRope => ropes.push(r.clone()),
                    crate::domain::Slot::PlanarSphere => spheres.push(r.clone()),
                    crate::domain::Slot::Dummy => todo!(),
                }
            }
        });

        let heads = self.deduplicate(heads, related_stats.clone())?;
        let hands = self.deduplicate(hands, related_stats.clone())?;
        let bodies = self.deduplicate(bodies, related_stats.clone())?;
        let feet = self.deduplicate(feet, related_stats.clone())?;
        let spheres = self.deduplicate(spheres, related_stats.clone())?;
        let ropes = self.deduplicate(ropes, related_stats)?;

        let heads_length = heads.len();
        let hands_length = hands.len();
        let bodies_length = bodies.len();
        let feet_length = feet.len();
        let spheres_length = spheres.len();
        let ropes_length = ropes.len();

        println!(
            "Number of relics to iterate over:\nHead: {:#?}\nHands: {:#?}\nBody: {:#?}\nFeet: {:#?}\nPlanar sphere: {:#?}\nLink rope: {:#?}",
            heads_length,
            hands_length,
            bodies_length,
            feet_length,
            spheres_length,
            ropes_length
        );

        let character: &mut Character = &mut self
            .characters
            .iter()
            .find(|c| c.key == character)
            .cloned()
            .ok_or(eyre::eyre!("Missing character: {character:?}"))?;

        // let mut ret = (vec![], StatDetails::default());
        // let current_target_val = match optimization_target {
        //     Stats::ATK => character.stats_panel.atk,
        //     Stats::ATK_ => todo!(),
        //     Stats::DEF => character.stats_panel.def,
        //     Stats::DEF_ => todo!(),
        //     Stats::HP => character.stats_panel.hp,
        //     Stats::HP_ => todo!(),
        //     Stats::CRITRate_ => character.stats_panel.crit_rate,
        //     Stats::CRITDMG_ => character.stats_panel.crit_dmg,
        //     Stats::SPD => character.stats_panel.spd,
        //     Stats::SPD_ => todo!(),
        //     Stats::EnergyRegenerationRate_ => character.stats_panel.energy_regeneration,
        //     Stats::EffectHitRate_ => character.stats_panel.effect_hit_rate,
        //     Stats::EffectRES_ => character.stats_panel.effect_res,
        //     Stats::BreakEffect_ => character.stats_panel.break_effect,
        //     Stats::OutgoingHealingBoost_ => character.stats_panel.healing_bonus,
        //     Stats::FireDMGBoost_
        //     | Stats::IceDMGBoost_
        //     | Stats::WindDMGBoost_
        //     | Stats::LightningDMGBoost_
        //     | Stats::QuantumDMGBoost_
        //     | Stats::ImaginaryDMGBoost_
        //     | Stats::PhysicalDMGBoost_ => character.stats_panel.dmg_bonus,
        //     Stats::Dummy => todo!(),
        // };

        let mut pb = kdam::tqdm!(
            total = heads_length
                * hands_length
                * bodies_length
                * feet_length
                * spheres_length
                * ropes_length
        );

        let mut async_task: Vec<tokio::task::JoinHandle<eyre::Result<Character>>> = vec![];
        for head in &heads {
            for hand in &hands {
                for body in &bodies {
                    for foot in &feet {
                        for sphere in &spheres {
                            for rope in &ropes {
                                let head = head.clone();
                                let hand = hand.clone();
                                let body = body.clone();
                                let foot = foot.clone();
                                let sphere = sphere.clone();
                                let rope = rope.clone();
                                let mut character = character.clone();
                                async_task.push(tokio::spawn(async move {
                                    character
                                        .update(
                                            vec![head, hand, body, foot, sphere, rope],
                                            character.light_cone.clone(),
                                        )
                                        .await?;
                                    Ok(character)
                                }));
                                pb.update(1)?;
                            }
                        }
                    }
                }
            }
        }

        let res = futures::future::join_all(async_task).await;

        let res = res
            .iter()
            .map(|r| {
                let character = r
                    .as_ref()
                    .map_err(|e| eyre::eyre!("{}", e))?
                    .as_ref()
                    .map_err(|e| eyre::eyre!("{}", e))?;
                let mut flag = true;
                min_requirement
                    .iter()
                    .for_each(|(stats, value)| match stats {
                        Stats::Atk => {
                            if character.stats_panel.atk < *value {
                                flag = false
                            }
                        }
                        Stats::Atk_ => todo!(),
                        Stats::Def => {
                            if character.stats_panel.def < *value {
                                flag = false
                            }
                        }
                        Stats::Def_ => todo!(),
                        Stats::Hp => {
                            if character.stats_panel.hp < *value {
                                flag = false
                            }
                        }
                        Stats::Hp_ => todo!(),
                        Stats::CritRate_ => {
                            if character.stats_panel.crit_rate < *value {
                                flag = false
                            }
                        }
                        Stats::CritDmg_ => {
                            if character.stats_panel.crit_dmg < *value {
                                flag = false
                            }
                        }
                        Stats::Spd => {
                            if character.stats_panel.spd < *value {
                                flag = false
                            }
                        }
                        Stats::Spd_ => todo!(),
                        Stats::EnergyRegenerationRate_ => {
                            if character.stats_panel.energy_regeneration < *value {
                                flag = false
                            }
                        }
                        Stats::EffectHitRate_ => {
                            if character.stats_panel.effect_hit_rate < *value {
                                flag = false
                            }
                        }
                        Stats::EffectRes_ => {
                            if character.stats_panel.effect_res < *value {
                                flag = false
                            }
                        }
                        Stats::BreakEffect_ => {
                            if character.stats_panel.break_effect < *value {
                                flag = false
                            }
                        }
                        Stats::OutgoingHealingBoost_ => {
                            if character.stats_panel.healing_bonus < *value {
                                flag = false
                            }
                        }
                        Stats::FireDmgBoost_
                        | Stats::IceDmgBoost_
                        | Stats::WindDmgBoost_
                        | Stats::LightningDmgBoost_
                        | Stats::QuantumDmgBoost_
                        | Stats::ImaginaryDmgBoost_
                        | Stats::PhysicalDmgBoost_ => {
                            if character.stats_panel.dmg_bonus < *value {
                                flag = false
                            }
                        }
                        Stats::Dummy => todo!(),
                        _ => (),
                    });
                Ok(if flag { Some(character.clone()) } else { None })
            })
            .collect::<eyre::Result<Vec<Option<Character>>>>()?;

        let mut res: Vec<Character> = res.iter().filter_map(|c| (*c).clone()).collect();
        res.sort_by(|x, y| {
            x.stats_panel
                .healing_bonus
                .total_cmp(&y.stats_panel.healing_bonus)
        });
        let one = &res[res.len() - 1];

        eprintln!();
        Ok((
            vec![
                one.relics.head.clone().unwrap_or_default(),
                one.relics.hands.clone().unwrap_or_default(),
                one.relics.body.clone().unwrap_or_default(),
                one.relics.feet.clone().unwrap_or_default(),
                one.relics.planar_sphere.clone().unwrap_or_default(),
                one.relics.link_rope.clone().unwrap_or_default(),
            ],
            one.stats_panel.clone(),
        ))
    }
}
