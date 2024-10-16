use crate::domain::{Relic, Slot};
use eyre::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;

use super::evaluator::Evaluator;

pub struct SimulatedAnnealing {
    pub initial_temp: f64,
    pub cooling_rate: f32,
    pub min_temp: f64,
    pub aggresive_factor: f32,
    pub relic_pool: HashMap<Slot, Vec<Relic>>,
    pub evaluator: Evaluator,
}

impl SimulatedAnnealing {
    pub fn simulated_annealing(&self, initial_solution: &[Relic]) -> Result<Vec<Relic>> {
        let mut current_solution = initial_solution.to_owned();
        let mut best_solution = initial_solution.to_owned();
        let mut current_temp = self.initial_temp;

        while current_temp > self.min_temp {
            // Generate a more aggresive neighbor solution by making larger changes
            let mut neighbor = current_solution.clone();

            // Determine how many elements to change based on the temperature
            let num_changes = std::cmp::max(
                1,
                (neighbor.len() as f64
                    * self.aggresive_factor as f64
                    * (current_temp / self.initial_temp)) as usize,
            );

            for _ in 0..num_changes {
                let mut thread_rng = thread_rng();
                let index = thread_rng.gen_range(0..neighbor.len() - 1);
                let slot = &neighbor[index].slot;
                if let Some(candidates) = self.relic_pool.get(slot) {
                    if let Some(new_relic) = candidates.choose(&mut thread_rng) {
                        neighbor[index] = new_relic.clone();
                    }
                }
            }

            // Calculate fitness of the neighbor and the current solution
            let current_fitness = self.evaluator.evaluate(current_solution.clone())?;
            let neighbor_fitness = self.evaluator.evaluate(neighbor.clone())?;

            // Decide if we should accept the neighbor
            if neighbor_fitness > current_fitness {
                current_solution = neighbor;
            } else {
                // Accept worse solutions with a probability based on temperature
                let probability = ((neighbor_fitness - current_fitness) / current_temp).exp();
                if thread_rng().gen::<f64>() < probability {
                    current_solution = neighbor;
                }
            }

            // Update the best solution found so far
            if self.evaluator.evaluate(current_solution.clone())?
                > self.evaluator.evaluate(best_solution.clone())?
            {
                best_solution = current_solution.clone();
            }

            // Cool down the temperature
            current_temp *= self.cooling_rate as f64;
        }
        Ok(best_solution)
    }
}
