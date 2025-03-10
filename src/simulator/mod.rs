use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use action_bar::{advance_action_system, Position, Speed};
use battle_field::{apply_action_system, choose_action_and_target_system, TargetAndAction};
use characters::{acheron_basic_attack, acheron_skill, acheron_ultimate, Character};
use enemies::Enemy;
use legion::{
    systems::CommandBuffer, world::SubWorld, Entity, IntoQuery, Resources, Schedule, World,
};

use crate::domain::{
    self, CharacterEntity, CharacterSkills, CharacterTraces, LightCone, LightConeEntity, Path,
    Stats,
};

pub mod action_bar;
pub mod battle_field;
pub mod characters;
pub mod enemies;
pub mod input_interface;

pub fn run() {
    let action: Arc<
        Mutex<
            dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer) + Send + Sync,
        >,
    > = Arc::new(Mutex::new(
        |character: &Entity,
         entity: &Entity,
         entities: &[Entity],
         world: &mut SubWorld,
         command: &mut CommandBuffer| {
            println!(
                "Attacking main target: {:?}, enemies: {:?}",
                entity, entities
            );
        },
    ));
    let acheron_basic_attack: Arc<
        Mutex<
            dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer) + Send + Sync,
        >,
    > = Arc::new(Mutex::new(acheron_basic_attack));
    let acheron_skill: Arc<
        Mutex<
            dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer) + Send + Sync,
        >,
    > = Arc::new(Mutex::new(acheron_skill));
    let acheron_ultimate: Arc<
        Mutex<
            dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer) + Send + Sync,
        >,
    > = Arc::new(Mutex::new(acheron_ultimate));
    let target_list: Arc<Mutex<dyn FnMut(&mut SubWorld) -> Vec<Entity> + Send + Sync>> =
        Arc::new(Mutex::new(|world: &mut SubWorld| -> Vec<Entity> {
            let mut query = <(Entity, &Enemy)>::query();
            query.iter(world).map(|(e, c)| e.clone()).collect()
        }));
    let enemy_target_list: Arc<Mutex<dyn FnMut(&mut SubWorld) -> Vec<Entity> + Send + Sync>> =
        Arc::new(Mutex::new(|world: &mut SubWorld| -> Vec<Entity> {
            let mut query = <(Entity, &CharacterEntity)>::query();
            query.iter(world).map(|(e, c)| e.clone()).collect()
        }));
    let mut world = World::default();
    let entity = world.push((
        Speed { speed: 100.0 },
        Position { position: 0.0 },
        TargetAndAction {
            target_and_action: HashMap::from([
                (
                    "Basic attack".to_string(),
                    (acheron_basic_attack, target_list.clone()),
                ),
                ("Skill".to_string(), (acheron_skill, target_list.clone())),
                (
                    "Ultimate".to_string(),
                    (acheron_ultimate, target_list.clone()),
                ),
            ]),
        },
        CharacterEntity {
            base_hp: 1125.43,
            base_atk: 698.54,
            base_def: 436.59,
            base_spd: 101.00,
            _base_aggro: 100,
            critical_chance: 5.0,
            critical_damage: 50.0,
            stat_bonus: HashMap::from([
                (Stats::LightningDmgBoost_, 8.0),
                (Stats::CritDmg_, 24.0),
                (Stats::Atk_, 28.0),
            ]),
            _character: domain::character::Character {
                id: "1308".to_string(),
                name: "Acheron".to_string(),
                path: Path::Nihility,
                level: 80,
                ascension: 6,
                eidolon: 0,
                skills: CharacterSkills {
                    basic: 1,
                    skill: 9,
                    ult: 9,
                    talent: 9,
                },
                traces: CharacterTraces {
                    ability_1: true,
                    ability_2: true,
                    ability_3: true,
                    stat_1: true,
                    stat_2: true,
                    stat_3: true,
                    stat_4: true,
                    stat_5: true,
                    stat_6: true,
                    stat_7: true,
                    stat_8: true,
                    stat_9: true,
                    stat_10: true,
                },
            },
        },
        LightConeEntity {
            base_hp: 952.56,
            base_atk: 476.28,
            base_def: 330.75,
            _light_cone: LightCone {
                id: "21001".to_string(),
                name: "Good Night and Sleep Well".to_string(),
                level: 80,
                ascension: 6,
                superimposition: 1,
                location: Some("1308".to_string()),
                lock: true,
                _uid: "light_cone_1".to_string(),
            },
        },
    ));
    let enemy = world.push((
        Speed { speed: 160.0 },
        Position { position: 0.0 },
        TargetAndAction {
            target_and_action: HashMap::from([(
                "Random".to_string(),
                (action.clone(), enemy_target_list.clone()),
            )]),
        },
        Enemy { level: 80 },
    ));
    let mut schedule = Schedule::builder()
        .add_system(advance_action_system())
        .flush()
        .add_system(choose_action_and_target_system())
        .flush()
        .add_system(apply_action_system())
        .flush()
        .build();

    let mut resources = Resources::default();

    while !<&CharacterEntity>::query()
        .iter(&world)
        .collect::<Vec<_>>()
        .is_empty()
        && !<&Enemy>::query()
            .iter(&world)
            .collect::<Vec<_>>()
            .is_empty()
    {
        schedule.execute(&mut world, &mut resources);
    }
}
