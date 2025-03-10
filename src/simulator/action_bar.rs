use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    sync::{Arc, Mutex},
};

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query};

pub const BAR_LENGTH: f32 = 10000.0;

pub trait Unit: Debug {
    fn get_speed(&self) -> f32;
    fn get_position(&self) -> f32;
    fn set_position(&self, new_pos: f32);
}

#[derive(Debug, Clone)]
pub struct Event {
    event_time: f32,
    unit_id: usize,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.event_time == other.event_time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.event_time.partial_cmp(&other.event_time)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ActionBar {
    pub units: HashMap<usize, Arc<Mutex<dyn Unit>>>,
    global_time: f32,
    events: BinaryHeap<Reverse<Event>>,
}

impl ActionBar {
    pub fn new(units: HashMap<usize, Arc<Mutex<dyn Unit>>>) -> Self {
        let mut ab = Self {
            global_time: 0.0,
            units,
            events: BinaryHeap::new(),
        };
        ab.rebuild_events();
        ab
    }

    fn rebuild_events(&mut self) {
        self.events.clear();
        for (unit_id, unit) in &self.units {
            let spd = unit.lock().unwrap().get_speed();
            if spd > 0.0 {
                let event_time =
                    self.global_time + (BAR_LENGTH - unit.lock().unwrap().get_position()) / spd;
                self.events.push(Reverse(Event {
                    event_time,
                    unit_id: *unit_id,
                }));
            }
        }
    }

    pub fn simulate_step(&mut self) -> Option<Arc<Mutex<dyn Unit>>> {
        if let Some(Reverse(event)) = self.events.pop() {
            let dt = event.event_time - self.global_time;
            self.global_time = event.event_time;

            for unit in self.units.values_mut() {
                unit.lock().unwrap().set_position(
                    unit.lock().unwrap().get_position() + unit.lock().unwrap().get_speed() * dt,
                );
            }

            let acted_unit = self.units.get(&event.unit_id).cloned();
            acted_unit
                .as_ref()
                .map(|unit| unit.lock().unwrap().set_position(0.0));
            self.rebuild_events();
            acted_unit
        } else {
            None
        }
    }

    pub fn push(&mut self, target_id: usize, percentage: f32) {
        if let Some(unit) = self.units.get_mut(&target_id) {
            let delta = BAR_LENGTH * (percentage / 100.0);
            unit.lock()
                .unwrap()
                .set_position(if unit.lock().unwrap().get_position() < delta {
                    0.0
                } else {
                    unit.lock().unwrap().get_position() - delta
                });
            self.rebuild_events();
        }
    }

    pub fn pull(&mut self, target_id: usize, percentage: f32) -> Option<Arc<Mutex<dyn Unit>>> {
        if let Some(unit) = self.units.get_mut(&target_id) {
            let delta = BAR_LENGTH * (percentage / 100.0);
            unit.lock()
                .unwrap()
                .set_position(unit.lock().unwrap().get_position() + delta);
            if unit.lock().unwrap().get_position() >= BAR_LENGTH {
                let acted_unit = unit.clone();
                unit.lock()
                    .unwrap()
                    .set_position(unit.lock().unwrap().get_position() - BAR_LENGTH);
                self.rebuild_events();
                return Some(acted_unit);
            }
            self.rebuild_events();
        }
        None
    }
}

pub struct Speed {
    pub speed: f32,
}

pub struct Position {
    pub position: f32,
}

pub struct ReadyForAction {}

#[system]
pub fn advance_action(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    query: &mut Query<(Entity, &Speed, &mut Position)>,
) {
    let mut shortest_time = f32::MAX;
    for (_, speed, position) in query.iter_mut(world) {
        let distance_left = BAR_LENGTH - position.position;
        let time_required = distance_left / speed.speed;
        if time_required < shortest_time {
            shortest_time = time_required;
        }
    }
    for (entity, speed, position) in query.iter_mut(world) {
        position.position = position.position + speed.speed * shortest_time;
        if position.position >= BAR_LENGTH {
            position.position -= BAR_LENGTH;
            commands.add_component(*entity, ReadyForAction {});
        }
    }
}
