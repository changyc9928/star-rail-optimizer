use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    sync::{Arc, Mutex},
};

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query};

use crate::{domain::CharacterEntity, simulator::{action_bar::ReadyForAction, characters::{Atk, Broken, CritDmg, CritRate, Def, DefIgnore, DefReduction, DmgBoost, DmgMitigation, ResPenalty, Resistance, Vulnerebility, Weaken}, enemies::Enemy}};

use super::{
    action_bar::{ActionBar, Unit},
    characters::{Character, CharacterTrait},
    enemies::{self, EnemyTrait},
};

pub struct BattleField {
    action_bar: ActionBar,
    characters: Vec<Arc<Mutex<dyn CharacterTrait>>>,
    enemies: Vec<Arc<Mutex<dyn EnemyTrait>>>,
}

impl BattleField {
    pub fn new(
        characters: Vec<Arc<Mutex<dyn CharacterTrait>>>,
        enemies: Vec<Arc<Mutex<dyn EnemyTrait>>>,
    ) -> Self {
        let mut units = HashMap::new();
        for character in &characters {
            let character_id = character.lock().unwrap().get_character_id() as usize;
            units.insert(
                character_id,
                character.lock().unwrap().get_unit_trait_object(),
            );
        }
        for enemy in &enemies {
            let enemy_id = enemy.lock().unwrap().get_enemy_id() as usize;
            units.insert(enemy_id, enemy.lock().unwrap().get_unit_trait_object());
        }
        let action_bar = ActionBar::new(units);
        Self {
            action_bar,
            characters,
            enemies,
        }
    }

    pub fn run(&mut self) {
        let unit = self.action_bar.simulate_step();
        let unit = if let Some(unit) = unit {
            unit
        } else {
            return;
        };
    }
}

pub struct TargetAndAction {
    pub target_and_action: HashMap<
        String,
        (
            Arc<
                Mutex<
                    dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer)
                        + Send
                        + Sync,
                >,
            >,
            Arc<Mutex<dyn FnMut(&mut SubWorld) -> Vec<Entity> + Send + Sync>>,
        ),
    >,
}

#[derive(Clone)]
pub struct ChosenAction {
    pub action: Arc<
        Mutex<
            dyn FnMut(&Entity, &Entity, &[Entity], &mut SubWorld, &mut CommandBuffer) + Send + Sync,
        >,
    >,
    pub main_target: Entity,
    pub possible_targets: Vec<Entity>,
}

#[system]
#[read_component(Enemy)]
#[read_component(CharacterEntity)]
#[read_component(Def)]
#[read_component(Atk)]
#[read_component(CritRate)]
#[read_component(CritDmg)]
#[read_component(Enemy)]
#[read_component(DmgBoost)]
#[read_component(Weaken)]
#[read_component(Resistance)]
#[read_component(Vulnerebility)]
#[read_component(DefReduction)]
#[read_component(DefIgnore)]
#[read_component(ResPenalty)]
#[read_component(DmgMitigation)]
#[read_component(Broken)]
pub fn apply_action(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    query: &mut Query<(Entity, &ChosenAction, &ReadyForAction)>,
) {
    let mut entity_b = None;
    let mut action_b = None;
    for (entity, action, _) in query.iter_mut(world) {
        entity_b = Some(entity.clone());
        action_b = Some(action.clone());
    }
    let entity = entity_b.expect("Should have an actioning unit");
    let action = action_b.expect("Should have an action selected");
    (action.action.lock().unwrap())(
        &entity,
        &action.main_target,
        &action.possible_targets,
        world,
        commands,
    );
    commands.remove_component::<ChosenAction>(entity);
    commands.remove_component::<ReadyForAction>(entity);
}

#[system]
#[read_component(CharacterEntity)]
#[read_component(Enemy)]
pub fn choose_action_and_target(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    query: &mut Query<(Entity, &ReadyForAction, &TargetAndAction)>,
) {
    let mut target_function = None;
    let mut choosen_action = None;
    let mut actioning_entity = None;
    for (unit, _, tna) in query.iter_mut(world) {
        let mut user_input_action = String::new();
        let option = tna.target_and_action.keys().collect::<Vec<_>>();
        println!("Possible actions: {:#?}", option);
        print!("Please enter action: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_input_action)
            .expect("Did not enter a correct string");
        if let Some('\n') = user_input_action.chars().next_back() {
            user_input_action.pop();
        }
        if let Some('\r') = user_input_action.chars().next_back() {
            user_input_action.pop();
        }
        println!("You typed: {}", user_input_action);

        let action = tna
            .target_and_action
            .get(&user_input_action)
            .expect("Invalid action entered");
        choosen_action = Some(action.0.clone());
        target_function = Some(action.1.clone());
        actioning_entity = Some(unit.clone());
    }

    let target_function = target_function.expect("Target listing function not found");
    let choosen_action = choosen_action.expect("Action function not found");
    let actioning_entity = actioning_entity.expect("Actioning unit not found");

    let possible_entities = target_function.lock().unwrap()(world);
    let mut user_input_target = String::new();
    println!("Possible targets:");
    println!("{:#?}", possible_entities);
    print!("Please choose the main target: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_input_target)
        .expect("Did not enter a correct string");
    if let Some('\n') = user_input_target.chars().next_back() {
        user_input_target.pop();
    }
    if let Some('\r') = user_input_target.chars().next_back() {
        user_input_target.pop();
    }
    println!("You typed: {}", user_input_target);

    commands.add_component(
        actioning_entity,
        ChosenAction {
            action: choosen_action,
            main_target: *possible_entities
                .get(
                    user_input_target
                        .parse::<usize>()
                        .expect("Input is not an integer"),
                )
                .expect("Invalid target"),
            possible_targets: possible_entities,
        },
    );
}
