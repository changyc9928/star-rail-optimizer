use super::simulated_annealing::SimulatedAnnealing;
use crate::{
    character::{Evaluator, Support},
    domain::{Enemy, Relic, RelicSetConfig, Relics, Slot},
};
use core::f64;
use eyre::{OptionExt, Result};
use rand::{
    rngs::ThreadRng,
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};
use rayon::prelude::*;
use std::{
    cmp::{min, Ordering},
    collections::HashMap,
    fmt::Display,
    sync::Arc,
};
use strum::IntoEnumIterator;
use tracing::info;

/// `Optimizer` struct used for optimizing relic sets in the context of a game or simulation.
/// It performs evolutionary optimization to find the best combination of relics based on the provided `Evaluator`.
pub struct Optimizer<T> {
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
    pub evaluator: Arc<dyn Evaluator<Target = T> + Sync + Send>,
    pub enable_sa: bool,
    pub simulated_annealing: SimulatedAnnealing<T>,

    pub target: T,
    pub enemy: Enemy,
    pub teammates: Vec<Box<dyn Support>>,
    pub relic_set_config: RelicSetConfig,
}

impl<T: Sync + Display> Optimizer<T> {
    // Helper method to calculate fitness sum
    #[allow(dead_code)]
    fn total_fitness(&self, population: &[Relics]) -> Result<f64> {
        let mut total = 0.0;
        for individual in population {
            let fitness =
                self.evaluator
                    .evaluate(individual, &self.enemy, &self.target, &self.teammates)?;
            total += fitness;
        }
        Ok(total)
    }

    // Roulette Wheel Selection method
    #[allow(dead_code)]
    fn roulette_wheel_selection(
        &self,
        population: &[Relics],
        rng: &mut ThreadRng,
    ) -> Result<Vec<Relics>> {
        let total_fitness = self.total_fitness(population)?;
        let mut cumulative_probabilities = Vec::with_capacity(population.len());
        let mut cumulative_sum = 0.0;

        // Calculate cumulative probabilities
        for individual in population {
            let fitness =
                self.evaluator
                    .evaluate(individual, &self.enemy, &self.target, &self.teammates)?;
            cumulative_sum += fitness / total_fitness;
            cumulative_probabilities.push(cumulative_sum);
        }

        let mut selected_population = Vec::with_capacity(population.len());
        while selected_population.len() < population.len() / 2 {
            let r = rng.gen::<f64>();
            for (i, &prob) in cumulative_probabilities.iter().enumerate() {
                if r < prob {
                    selected_population.push(population[i].clone());
                    break;
                }
            }
        }

        Ok(selected_population)
    }

    fn evaluation(&self, x: &Relics, y: &Relics) -> Ordering {
        match (
            self.evaluator
                .evaluate(x, &self.enemy, &self.target, &self.teammates),
            self.evaluator
                .evaluate(y, &self.enemy, &self.target, &self.teammates),
        ) {
            (Ok(x_val), Ok(y_val)) => x_val.partial_cmp(&y_val).unwrap(),
            _ => f64::MIN.partial_cmp(&f64::MIN).unwrap(),
        }
    }

    fn tournament_selection(
        &self,
        population: &[Relics],
        tournament_size: usize,
    ) -> Result<Vec<Relics>> {
        let selected = (0..self.population_size / 2)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let tournament: Vec<&Relics> = (0..tournament_size)
                    .map(|_| population.choose(&mut rng).unwrap())
                    .collect();
                let winner = tournament
                    .par_iter()
                    .max_by(|arg0, arg1| self.evaluation(arg0, arg1))
                    .ok_or_else(|| eyre::eyre!("No winner found in tournament"))?;
                Ok::<_, eyre::Report>(winner.clone().clone())
            })
            .collect::<Result<Vec<Relics>>>()?;
        Ok(selected)
    }

    /// Starts the optimization process and returns the best relic set found.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Relic>)` - The best relic set found after all generations.
    /// - `Err(e)` - An error if something goes wrong during the optimization process.
    pub fn optimize(&self) -> Result<Relics> {
        // Initialize the population with random relic sets.
        let mut population: Vec<Relics> = (0..self.population_size)
            .map(|_| self.generate_random_relic_set())
            .collect();

        // Run the optimization process over a number of generations.
        for generation in 0..self.generation {
            // Use Roulette Wheel Selection to select parents
            let mut selected_population = self.tournament_selection(&population, 5)?; // 5 is the tournament size

            let difference = (self.population_size - selected_population.len()) / 2;

            // Generate new individuals through crossover and mutation in parallel.
            let mut new_gen: Vec<Relics> = (0..difference)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    // Randomly select two parents from the selected population
                    let parents = selected_population
                        .clone()
                        .into_iter()
                        .choose_multiple(&mut rng, 2);

                    let children = self.crossover(parents)?;

                    // Apply mutation to the children and add them to the next generation.
                    let mutated_children: Vec<_> = children
                        .into_iter()
                        .map(|child| self.mutate(child))
                        .collect::<Result<_>>()?;

                    Ok::<Vec<_>, eyre::Report>(mutated_children)
                })
                .collect::<Result<Vec<_>>>()?
                .into_par_iter()
                .flatten()
                .collect();

            selected_population.append(&mut new_gen);

            // Update the population for the next generation.
            population = selected_population;

            if self.enable_sa {
                // Apply Simulated Annealing to the best solution found every few generations
                if generation % 10 == 0 {
                    // Find the best individual in the current population
                    let mut best_individual = population
                        .par_iter()
                        .max_by(|arg0, arg1| self.evaluation(arg0, arg1))
                        .ok_or_eyre("Best combination not found")?
                        .clone();
                    let best_fit = self.evaluator.evaluate(
                        &best_individual,
                        &self.enemy,
                        &self.target,
                        &self.teammates,
                    )?;
                    info!(
                        "Generation {generation}, before SA, Highest {}: {}",
                        self.target, best_fit
                    );

                    // Apply aggresive SA
                    best_individual = self
                        .simulated_annealing
                        .simulated_annealing(&best_individual)?;
                    let best_fit = self.evaluator.evaluate(
                        &best_individual,
                        &self.enemy,
                        &self.target,
                        &self.teammates,
                    )?;
                    info!(
                        "Generation {generation}, after SA, Highest {}: {}",
                        self.target, best_fit
                    );
                    let random_index = thread_rng().gen_range(0..population.len() - 1);
                    population[random_index] = best_individual;
                }
            }

            // Find and print the best relic set of the current generation in parallel.
            let best_combination = population
                .par_iter()
                .max_by(|arg0, arg1| self.evaluation(arg0, arg1))
                .ok_or_eyre("Best combination not found")?;

            let result = self.evaluator.evaluate(
                best_combination,
                &self.enemy,
                &self.target,
                &self.teammates,
            )?;
            info!(
                "Generation {} Highest {}: {}",
                generation + 1,
                self.target,
                result
            );
        }

        // Sort the final population and return the best relic set.
        population.par_sort_by(|x, y| self.evaluation(x, y));
        let best_relic_set = population.last().unwrap().clone();
        Ok(best_relic_set)
    }

    /// Generates a random relic set by selecting one relic for each slot from the available relic pool.
    ///
    /// # Returns
    ///
    /// - `Vec<Relic>` - A vector of randomly selected relics for each slot.
    fn generate_random_relic_set(&self) -> Relics {
        // Collect all the slots
        let slots: Vec<Slot> = Slot::iter().collect();

        // Generate relics in parallel
        let relics: Vec<Relic> = slots
            .iter()
            .filter_map(|slot| {
                let mut rng = thread_rng(); // Create a new RNG instance for each thread
                if let Some(relics_for_slot) = self.relic_pool.get(slot) {
                    relics_for_slot.iter().choose(&mut rng).cloned()
                } else {
                    None
                }
            })
            .collect();

        Relics {
            relics,
            config: self.relic_set_config.clone(),
        }
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
    fn crossover(&self, parents: Vec<Relics>) -> Result<Vec<Relics>> {
        let mut parents = parents.iter();
        let parent1 = parents.next().ok_or(eyre::eyre!("Missing parent 1"))?;
        let parent2 = parents.next().ok_or(eyre::eyre!("Missing parent 2"))?;

        let min_length = min(parent1.relics.len(), parent2.relics.len());

        let mut child1 = vec![];
        let mut child2 = vec![];

        let mut rng = thread_rng();
        for i in 0..min_length {
            if rng.gen::<f64>() > self.crossover_rate {
                child1.push(parent1.relics[i].clone());
                child2.push(parent2.relics[i].clone());
            } else {
                child1.push(parent2.relics[i].clone());
                child2.push(parent1.relics[i].clone());
            }
        }

        // Append the remaining relics if the parents have unequal lengths.
        if parent1.relics.len() > min_length {
            child1.extend_from_slice(&parent1.relics[min_length..]);
            child2.extend_from_slice(&parent1.relics[min_length..]);
        } else {
            child1.extend_from_slice(&parent2.relics[min_length..]);
            child2.extend_from_slice(&parent2.relics[min_length..]);
        }

        Ok(vec![
            Relics {
                relics: child1,
                config: self.relic_set_config.clone(),
            },
            Relics {
                relics: child2,
                config: self.relic_set_config.clone(),
            },
        ])
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
    fn mutate(&self, child: Relics) -> Result<Relics> {
        let mut mutated_child = child;

        // Parallelize the mutation of relics
        mutated_child.relics.iter_mut().for_each(|relic| {
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
