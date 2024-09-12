use super::evaluator::Evaluator;
use crate::domain::{Relic, Slot};
use core::f64;
use eyre::{OptionExt, Result};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};
use rayon::prelude::*;
use std::{cmp::min, collections::HashMap};
use strum::IntoEnumIterator;
use tracing::info;

/// `Optimizer` struct used for optimizing relic sets in the context of a game or simulation.
/// It performs evolutionary optimization to find the best combination of relics based on the provided `Evaluator`.
pub struct Optimizer {
    /// A mapping of slots to a vector of possible relics for each slot.
    pub relic_pool: HashMap<Slot, Vec<Relic>>,
    /// The number of generations to run the optimization process.
    pub generation: usize,
    /// The size of the population in each generation.
    pub population_size: usize,
    /// The probability of mutation occurring during the mutation phase.
    pub mutation_rate: f64,
    /// The probability of performing crossover between parent relic sets.
    pub crossover_rate: f64,
    /// An `Evaluator` instance used to evaluate the fitness of relic sets.
    pub evaluator: Evaluator,
}

impl Optimizer {
    /// Starts the optimization process and returns the best relic set found.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Relic>)` - The best relic set found after all generations.
    /// - `Err(e)` - An error if something goes wrong during the optimization process.
    pub fn optimize(&self) -> Result<Vec<Relic>> {
        // Initialize the population with random relic sets.
        let mut population: Vec<Vec<Relic>> = (0..self.population_size)
            .map(|_| self.generate_random_relic_set())
            .collect();
        let mut rng = thread_rng();

        // Define an evaluation function for comparing two relic sets.
        let evaluation = |x: &Vec<Relic>, y: &Vec<Relic>| match (
            self.evaluator.evaluate(y.clone()),
            self.evaluator.evaluate(x.clone()),
        ) {
            (Ok(y_val), Ok(x_val)) => y_val.partial_cmp(&x_val).unwrap(),
            _ => f64::MIN.partial_cmp(&f64::MIN).unwrap(),
        };

        // Run the optimization process over a number of generations.
        for generation in 0..self.generation {
            // Sort the population based on the evaluation function in parallel.
            population.par_sort_unstable_by(evaluation);

            // Keep the top half of the population for the next generation.
            let mut next_generation = population[..self.population_size / 2].to_vec();

            // Generate new individuals through crossover and mutation in parallel.
            while next_generation.len() < self.population_size {
                // Randomly select two parents from the top population.
                let parents = next_generation
                    .clone()
                    .into_iter()
                    .choose_multiple(&mut rng, 2);

                let children = self.crossover(parents)?;

                // Apply mutation to the children and add them to the next generation.
                let mut mutated_children: Vec<_> = children
                    .into_par_iter()
                    .map(|child| self.mutate(child))
                    .collect::<Result<_>>()?;

                next_generation.append(&mut mutated_children);
            }

            // Update the population for the next generation.
            population = next_generation;

            // Find and print the best relic set of the current generation in parallel.
            let best_combination = population
                .par_iter()
                .max_by(|arg0: &&Vec<Relic>, arg1: &&Vec<Relic>| evaluation(*arg0, *arg1))
                .ok_or_eyre("Best combination not found")?;

            let result = self.evaluator.evaluate(best_combination.clone())?;
            info!(
                "Generation {} Highest {}: {}",
                generation + 1,
                self.evaluator.target_name,
                result
            );
        }

        // Sort the final population and return the best relic set.
        population.par_sort_by(evaluation);
        let best_relic_set = population.first().unwrap().clone();
        Ok(best_relic_set)
    }

    /// Generates a random relic set by selecting one relic for each slot from the available relic pool.
    ///
    /// # Returns
    ///
    /// - `Vec<Relic>` - A vector of randomly selected relics for each slot.
    fn generate_random_relic_set(&self) -> Vec<Relic> {
        // Collect all the slots
        let slots: Vec<Slot> = Slot::iter().collect();

        // Generate relics in parallel
        let relics: Vec<Relic> = slots
            .par_iter()
            .filter_map(|slot| {
                let mut rng = thread_rng(); // Create a new RNG instance for each thread
                if let Some(relics_for_slot) = self.relic_pool.get(slot) {
                    relics_for_slot.iter().choose(&mut rng).cloned()
                } else {
                    None
                }
            })
            .collect();

        relics
    }

    /// Performs crossover between two parent relic sets to produce two child relic sets.
    ///
    /// # Parameters
    ///
    /// - `parents` - A vector containing two parent relic sets.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Vec<Relic>>)` - A vector containing two child relic sets resulting from the crossover.
    /// - `Err(e)` - An error if there are not exactly two parents provided.
    fn crossover(&self, parents: Vec<Vec<Relic>>) -> Result<Vec<Vec<Relic>>> {
        let mut parents = parents.iter();
        let parent1 = parents.next().ok_or(eyre::eyre!("Missing parent 1"))?;
        let parent2 = parents.next().ok_or(eyre::eyre!("Missing parent 2"))?;

        let min_length = min(parent1.len(), parent2.len());

        let mut child1 = vec![];
        let mut child2 = vec![];

        let mut rng = thread_rng();
        for i in 0..min_length {
            if rng.gen::<f64>() > self.crossover_rate {
                child1.push(parent1[i].clone());
                child2.push(parent2[i].clone());
            } else {
                child1.push(parent2[i].clone());
                child2.push(parent1[i].clone());
            }
        }

        // Append the remaining relics if the parents have unequal lengths.
        if parent1.len() > min_length {
            child1.extend_from_slice(&parent1[min_length..]);
            child2.extend_from_slice(&parent1[min_length..]);
        } else {
            child1.extend_from_slice(&parent2[min_length..]);
            child2.extend_from_slice(&parent2[min_length..]);
        }

        Ok(vec![child1, child2])
    }

    /// Applies mutation to a relic set by randomly changing some of its relics.
    ///
    /// # Parameters
    ///
    /// - `child` - The relic set to mutate.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Relic>)` - The mutated relic set.
    /// - `Err(e)` - An error if a relic's slot is not found in the relic pool.
    fn mutate(&self, child: Vec<Relic>) -> Result<Vec<Relic>> {
        let mut mutated_child = child;

        // Parallelize the mutation of relics
        mutated_child.par_iter_mut().for_each(|relic| {
            let mut rng = rand::thread_rng(); // Create a new RNG for each thread
            if rng.gen::<f64>() < self.mutation_rate {
                let slot = &relic.slot;
                if let Some(candidates) = self.relic_pool.get(slot) {
                    if let Some(new_relic) = candidates.choose(&mut rng) {
                        *relic = new_relic.clone();
                    }
                }
            }
        });

        Ok(mutated_child)
    }
}
