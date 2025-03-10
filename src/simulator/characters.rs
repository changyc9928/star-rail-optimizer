use std::sync::{Arc, Mutex};

use legion::{systems::CommandBuffer, world::SubWorld, Entity, EntityStore};
use rand::Rng;

use crate::domain::{CharacterEntity, LightConeEntity};

use super::{
    action_bar::{Position, Unit, BAR_LENGTH},
    enemies::Enemy,
};

pub trait CharacterTrait: Unit {
    fn get_character_id(&self) -> i64;

    fn get_unit_trait_object(&self) -> Arc<Mutex<dyn Unit>>;
}

pub struct Character {}

pub struct Hp {
    pub hp: f32,
}

pub struct Atk {
    pub atk: f32,
}

pub struct Def {
    pub def: f32,
}

pub struct CritRate {
    pub rate: f32,
}

pub struct CritDmg {
    pub crit_dmg: f32,
}

pub struct EnergyRegenerationRate {
    pub energy_regen_rate: f32,
}

pub struct Energy {
    pub energy: f32,
}

pub struct EffectHitRate {
    pub effect_hit_rate: f32,
}

pub struct BreakEffect {
    pub break_effect: f32,
}

pub struct Shield {
    pub shield: f32,
}

pub struct DmgBoost {
    pub lightning_boost: f32,
    pub physical_boost: f32,
    pub wind_boost: f32,
    pub ice_boost: f32,
    pub fire_boost: f32,
    pub quantum_boost: f32,
    pub imaginary_boost: f32,
    pub all_type_boost: f32,
    pub dot_boost: f32,
}

pub struct Weaken {
    pub weaken: f32,
}

pub struct Resistance {
    pub lightning_resistance: f32,
    pub physical_resistance: f32,
    pub wind_resistance: f32,
    pub ice_resistance: f32,
    pub fire_resistance: f32,
    pub quantum_resistance: f32,
    pub imaginary_resistance: f32,
}

pub struct Vulnerebility {
    pub lightning_vulnerability: f32,
    pub physical_vulnerability: f32,
    pub wind_vulnerability: f32,
    pub fire_vulnerability: f32,
    pub ice_vulnerability: f32,
    pub quantum_vulnerability: f32,
    pub imaginary_vulnerability: f32,
    pub all_type_vulnarability: f32,
    pub dot_vulnerability: f32,
}

pub struct DefReduction {
    pub def_reduction: f32,
}

pub struct DefIgnore {
    pub def_ignore: f32,
}

pub struct DmgMitigation {
    pub dmg_mitigation: Vec<f32>,
}

pub struct ResPenalty {
    pub res_penalty: f32,
}

#[derive(Clone)]
pub struct Broken {
    pub toughness: f32,
    pub broken: bool,
}

pub struct SlashedDream {
    pub stack: u8,
}

pub struct CrimsonKnot {
    pub stack: u8,
}

pub struct AcheronUltimatePhase {
    pub phase: u8,
}

pub fn acheron_basic_attack(
    character: &Entity,
    main_target: &Entity,
    enemies: &[Entity],
    world: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    let character_entry = world.entry_ref(*character).expect("Acheron is not found");
    let main_target_entry = world
        .entry_ref(*main_target)
        .expect("We need a main target to attack");
    let character_data = character_entry
        .get_component::<CharacterEntity>()
        .expect("A character should have its own data");
    let multiplier = [0.5, 0.5, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3]
        [character_data._character.skills.basic as usize];
    let (toughness, total_dmg) =
        calculate_damage(character, main_target, world, character_data, multiplier);
    let enemy_hp = main_target_entry
        .get_component::<Hp>()
        .expect("An enemy should have HP");
    let new_hp = enemy_hp.hp - total_dmg;
    let enemy_toughness = toughness.toughness;
    let new_toughness = enemy_toughness - 10.0;
    if new_hp > 0.0 {
        command.add_component(*main_target, Hp { hp: new_hp });
        command.add_component(
            *main_target,
            Broken {
                toughness: new_toughness,
                broken: new_toughness == 0.0,
            },
        );
    } else {
        command.remove(*main_target);
    }
}

fn calculate_damage(
    character: &Entity,
    main_target: &Entity,
    world: &SubWorld,
    character_data: &CharacterEntity,
    multiplier: f32,
) -> (Broken, f32) {
    let character_entry = world.entry_ref(*character).expect("Acheron is not found");
    let main_target_entry = world
        .entry_ref(*main_target)
        .expect("We need a main target to attack");
    let atk = character_entry
        .get_component::<Atk>()
        .expect("A character should have atk")
        .atk;
    let crit_rate = character_entry
        .get_component::<CritRate>()
        .expect("A character should have crit rate");
    let random_num: f32 = rand::thread_rng().gen();
    let crit_dmg = if random_num > crit_rate.rate / 100.0 {
        character_entry
            .get_component::<CritDmg>()
            .expect("A character should have crit dmg")
            .crit_dmg
            / 100.0
    } else {
        0.0
    };
    let dmg_boost = character_entry
        .get_component::<DmgBoost>()
        .expect("A character should have dmg boost");
    let weaken = character_entry
        .get_component::<Weaken>()
        .expect("A character should have weaken")
        .weaken
        / 100.0;
    let enemy_resistance = main_target_entry
        .get_component::<Resistance>()
        .expect("An enemy should have resistance");
    let enemy_vulnerability = main_target_entry
        .get_component::<Vulnerebility>()
        .expect("An enemy should have vulnerability");
    let enemy_data = main_target_entry
        .get_component::<Enemy>()
        .expect("An enemy should have data");
    let def_reduction = character_entry
        .get_component::<DefReduction>()
        .expect("A character should have def reduction");
    let def_ignore = character_entry
        .get_component::<DefIgnore>()
        .expect("A character should have def ignore");
    let res_penalty = character_entry
        .get_component::<ResPenalty>()
        .expect("A character should have res penalty");
    let dmg_mitigation = main_target_entry
        .get_component::<DmgMitigation>()
        .expect("An enemy should have dmg mitigation");
    let toughness = main_target_entry
        .get_component::<Broken>()
        .expect("An enemy should have toughness");

    let base_dmg = multiplier * atk;
    let crit_multiplier = 1.0 + crit_dmg;
    let dmg_boost_multiplier =
        1.0 + dmg_boost.all_type_boost / 100.0 + dmg_boost.lightning_boost / 100.0;
    let def_multiplier = (character_data._character.level + 20) as f32
        / ((enemy_data.level + 20) as f32
            * (1.0 - def_reduction.def_reduction / 100.0 - def_ignore.def_ignore / 100.0)
            + character_data._character.level as f32
            + 20.0);
    let res_multiplier =
        1.0 - (enemy_resistance.lightning_resistance / 100.0 - res_penalty.res_penalty / 100.0);
    let vulnerabilty_multiplier = 1.0
        + enemy_vulnerability.lightning_vulnerability / 100.0
        + enemy_vulnerability.all_type_vulnarability / 100.0;
    let dmg_mitigation = dmg_mitigation
        .dmg_mitigation
        .clone()
        .into_iter()
        .reduce(|acc, e| acc * (1.0 - e / 100.0))
        .expect("Unexpected error when calculating enemy dmg mitagation");
    let broken = if toughness.broken { 1.0 } else { 0.9 };
    let total_dmg = base_dmg
        * crit_multiplier
        * dmg_boost_multiplier
        * weaken
        * def_multiplier
        * res_multiplier
        * vulnerabilty_multiplier
        * dmg_mitigation
        * broken;
    (toughness.clone(), total_dmg)
}

pub fn acheron_skill(
    character: &Entity,
    main_target: &Entity,
    enemies: &[Entity],
    world: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    let character_entry = world
        .entry_ref(*character)
        .expect("Character should be existing");
    let current_slashed_dream = character_entry
        .get_component::<SlashedDream>()
        .expect("Acheron should have slashed dream");
    command.add_component(
        *character,
        SlashedDream {
            stack: current_slashed_dream.stack + 1,
        },
    );
    let main_target_entry = world
        .entry_ref(*main_target)
        .expect("Should have a main target to apply damage");
    let current_main_target_crimson_knot = main_target_entry
        .get_component::<CrimsonKnot>()
        .unwrap_or(&CrimsonKnot { stack: 0 });
    command.add_component(
        *main_target,
        CrimsonKnot {
            stack: current_main_target_crimson_knot.stack + 1,
        },
    );
    let character_data = character_entry
        .get_component::<CharacterEntity>()
        .expect("A character should have its own data");
    let multiplier = [
        0.8, 0.88, 0.96, 1.04, 1.12, 1.2, 1.3, 1.4, 1.5, 1.6, 1.68, 1.76, 1.84, 1.92, 2.0,
    ][character_data._character.skills.skill as usize];
    let (toughness, total_dmg) =
        calculate_damage(character, main_target, world, character_data, multiplier);
    let enemy_hp = main_target_entry
        .get_component::<Hp>()
        .expect("An enemy should have HP");
    let new_hp = enemy_hp.hp - total_dmg;
    let enemy_toughness = toughness.toughness;
    let new_toughness = enemy_toughness - 20.0;
    if new_hp > 0.0 {
        command.add_component(*main_target, Hp { hp: new_hp });
        command.add_component(
            *main_target,
            Broken {
                toughness: new_toughness,
                broken: new_toughness == 0.0,
            },
        );
    } else {
        command.remove(*main_target);
    }
    let main_target_position = enemies
        .iter()
        .position(|e| e == main_target)
        .expect("Should have main target found in the list of enemies");
    for (i, enemy) in enemies.iter().enumerate() {
        if i == main_target_position - 1 || i == main_target_position + 1 {
            let multiplier = [
                0.3, 0.33, 0.36, 0.39, 0.42, 0.45, 0.4875, 0.5250, 0.5625, 0.6, 0.63, 0.66, 0.69,
                0.72, 0.75,
            ][character_data._character.skills.skill as usize];
            let (toughness, total_dmg) =
                calculate_damage(character, enemy, world, character_data, multiplier);
            let enemy_entry = world.entry_ref(*enemy);
            if let Ok(enemy_entry) = enemy_entry {
                let enemy_hp = enemy_entry
                    .get_component::<Hp>()
                    .expect("An enemy should have HP");
                let new_hp = enemy_hp.hp - total_dmg;
                let enemy_toughness = toughness.toughness;
                let new_toughness = enemy_toughness - 10.0;
                if new_hp > 0.0 {
                    command.add_component(*enemy, Hp { hp: new_hp });
                    command.add_component(
                        *enemy,
                        Broken {
                            toughness: new_toughness,
                            broken: new_toughness == 0.0,
                        },
                    );
                } else {
                    command.remove(*enemy);
                }
            }
        }
    }
}

pub fn acheron_ultimate(
    character: &Entity,
    main_target: &Entity,
    enemies: &[Entity],
    world: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    let character_entry = world
        .entry_ref(*character)
        .expect("Character should be existing");
    let ultimate_phase = character_entry.get_component::<AcheronUltimatePhase>();
    if let Ok(ultimate_phase) = ultimate_phase {
        // phase 2 onwards
        if ultimate_phase.phase == 2 || ultimate_phase.phase == 3 {
            command.add_component(
                *character,
                AcheronUltimatePhase {
                    phase: ultimate_phase.phase + 1,
                },
            );
            let character_data = character_entry
                .get_component::<CharacterEntity>()
                .expect("A character should have its own data");
            let multiplier = [
                0.144, 0.1536, 0.1632, 0.1728, 0.1824, 0.1920, 0.2040, 0.2160, 0.2280, 0.24,
                0.2496, 0.2592, 0.2688, 0.2784, 0.288,
            ][character_data._character.skills.ult as usize];
            let (toughness, total_dmg) =
                calculate_damage(character, main_target, world, character_data, multiplier);
            let main_target_entry = world
                .entry_ref(*main_target)
                .expect("Should have a main target to apply damage");
            let enemy_hp = main_target_entry
                .get_component::<Hp>()
                .expect("An enemy should have HP");
            let mut new_hp = enemy_hp.hp - total_dmg;
            let enemy_toughness = toughness.toughness;
            let mut new_toughness = enemy_toughness - 5.0;
            let enemy_crinsom_knot =
                if let Ok(crinsom_knot) = main_target_entry.get_component::<CrimsonKnot>() {
                    crinsom_knot.stack
                } else {
                    0
                };
            let ck_multiplier = [
                0.09, 0.096, 0.102, 0.108, 0.114, 0.12, 0.1275, 0.1350, 0.1425, 0.15, 0.156,
                0.162, 0.168, 0.174, 0.18,
            ][character_data._character.skills.ult as usize]
                * (if enemy_crinsom_knot > 0 {
                    enemy_crinsom_knot + 1
                } else {
                    0
                }) as f32;
            let (_, ck_total_dmg) =
                calculate_damage(character, main_target, world, character_data, ck_multiplier);
            new_toughness -= 5.0;
            new_hp -= ck_total_dmg;
            if new_hp > 0.0 {
                command.add_component(*main_target, Hp { hp: new_hp });
                command.add_component(
                    *main_target,
                    Broken {
                        toughness: new_toughness,
                        broken: new_toughness == 0.0,
                    },
                );
            } else {
                command.remove(*main_target);
            }
            for enemy in enemies {
                let enemy_entry = world
                    .entry_ref(*enemy)
                    .expect("Should have a main target to apply damage");
                let enemy_hp = enemy_entry
                    .get_component::<Hp>()
                    .expect("An enemy should have HP");
                let (toughness, total_dmg) =
                    calculate_damage(character, enemy, world, character_data, ck_multiplier);
                let new_toughness = toughness.toughness - 5.0;
                let new_hp = enemy_hp.hp - total_dmg;
                if new_hp > 0.0 {
                    command.add_component(*enemy, Hp { hp: new_hp });
                    command.add_component(
                        *enemy,
                        Broken {
                            toughness: new_toughness,
                            broken: new_toughness == 0.0,
                        },
                    );
                } else {
                    command.remove(*enemy);
                }
            }
        } else if ultimate_phase.phase == 4 {
            for enemy in enemies {
                let enemy_entry = world
                    .entry_ref(*enemy)
                    .expect("Should have a main target to apply damage");
                let enemy_hp = enemy_entry
                    .get_component::<Hp>()
                    .expect("An enemy should have HP");
                let character_data = character_entry
                    .get_component::<CharacterEntity>()
                    .expect("A character should have its own data");
                let multiplier = [
                    0.72, 0.768, 0.816, 0.864, 0.912, 0.96, 1.02, 1.08, 1.14, 1.2, 1.24, 1.296,
                    1.344, 1.392, 1.44,
                ][character_data._character.skills.ult as usize];
                let (toughness, total_dmg) =
                    calculate_damage(character, enemy, world, character_data, multiplier);
                let new_toughness = toughness.toughness - 5.0;
                let new_hp = enemy_hp.hp - total_dmg;
                if new_hp > 0.0 {
                    command.add_component(*enemy, Hp { hp: new_hp });
                    command.add_component(
                        *enemy,
                        Broken {
                            toughness: new_toughness,
                            broken: new_toughness == 0.0,
                        },
                    );
                    command.remove_component::<CrimsonKnot>(*enemy);
                    command.remove_component::<AcheronUltimatePhase>(*character);
                } else {
                    command.remove(*enemy);
                }
            }
        }
    } else {
        // initial phase
        let slashed_dream = character_entry
            .get_component::<SlashedDream>()
            .expect("Acheron should have slashed dream");
        command.add_component(
            *character,
            SlashedDream {
                stack: slashed_dream.stack - 9,
            },
        );
        command.add_component(
            *character,
            Position {
                position: BAR_LENGTH,
            },
        );
        command.add_component(*character, AcheronUltimatePhase { phase: 2 });
        let character_data = character_entry
            .get_component::<CharacterEntity>()
            .expect("A character should have its own data");
        let multiplier = [
            0.144, 0.1536, 0.1632, 0.1728, 0.1824, 0.1920, 0.2040, 0.2160, 0.2280, 0.24, 0.2496,
            0.2592, 0.2688, 0.2784, 0.288,
        ][character_data._character.skills.ult as usize];
        let (toughness, total_dmg) =
            calculate_damage(character, main_target, world, character_data, multiplier);
        let main_target_entry = world
            .entry_ref(*main_target)
            .expect("Should have a main target to apply damage");
        let enemy_hp = main_target_entry
            .get_component::<Hp>()
            .expect("An enemy should have HP");
        let mut new_hp = enemy_hp.hp - total_dmg;
        let enemy_toughness = toughness.toughness;
        let mut new_toughness = enemy_toughness - 5.0;
        let enemy_crinsom_knot =
            if let Ok(crinsom_knot) = main_target_entry.get_component::<CrimsonKnot>() {
                crinsom_knot.stack
            } else {
                0
            };
        let ck_multiplier = [
            0.09, 00.096, 0.102, 0.108, 0.114, 0.12, 0.1275, 0.1350, 0.1425, 0.15, 0.156, 0.162,
            0.168, 0.174, 0.18,
        ][character_data._character.skills.ult as usize]
            * (if enemy_crinsom_knot > 0 {
                enemy_crinsom_knot + 1
            } else {
                0
            }) as f32;
        let (_, ck_total_dmg) =
            calculate_damage(character, main_target, world, character_data, ck_multiplier);
        new_toughness -= 5.0;
        new_hp -= ck_total_dmg;
        if new_hp > 0.0 {
            command.add_component(*main_target, Hp { hp: new_hp });
            command.add_component(
                *main_target,
                Broken {
                    toughness: new_toughness,
                    broken: new_toughness == 0.0,
                },
            );
        } else {
            command.remove(*main_target);
        }
        for enemy in enemies {
            let enemy_entry = world
                .entry_ref(*enemy)
                .expect("Should have a main target to apply damage");
            let enemy_hp = enemy_entry
                .get_component::<Hp>()
                .expect("An enemy should have HP");
            let (toughness, total_dmg) =
                calculate_damage(character, enemy, world, character_data, ck_multiplier);
            let new_toughness = toughness.toughness - 5.0;
            let new_hp = enemy_hp.hp - total_dmg;
            if new_hp > 0.0 {
                command.add_component(*enemy, Hp { hp: new_hp });
                command.add_component(
                    *enemy,
                    Broken {
                        toughness: new_toughness,
                        broken: new_toughness == 0.0,
                    },
                );
            } else {
                command.remove(*enemy);
            }
        }
    }
}
