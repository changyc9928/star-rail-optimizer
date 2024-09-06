use super::evaluator::Evaluator;
use crate::domain::{Relic, Slot};
use core::f64;
use eyre::Result;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};
use std::{
    cmp::{min, Reverse},
    collections::HashMap,
};
use strum::IntoEnumIterator;

pub struct Optimizer {
    pub relic_pool: HashMap<Slot, Vec<Relic>>,
    pub generation: usize,
    pub population_size: usize,
    pub mutation_rate: f64,
    pub evaluator: Evaluator,
}

impl Optimizer {
    pub fn optimize(&self) -> Result<Vec<Relic>> {
        let mut population: Vec<Vec<Relic>> = (0..self.population_size)
            .into_iter()
            .map(|_| self.generate_random_relic_set())
            .collect();
        let mut rng = thread_rng();

        let evaluation = |x: &Vec<Relic>, y: &Vec<Relic>| {
            self.evaluator
                .evaluate(y.clone())
                .unwrap_or(f64::MIN)
                .partial_cmp(&self.evaluator.evaluate(x.clone()).unwrap_or(f64::MIN))
                .unwrap()
        };

        for generation in 0..self.generation {
            population.sort_unstable_by(evaluation);
            let mut next_generation = population[..self.population_size / 2].to_vec();

            while next_generation.len() < self.population_size {
                let parents = next_generation
                    .clone()
                    .into_iter()
                    .choose_multiple(&mut rng, 2);
                let children = self.crossover(parents)?;

                next_generation.push(self.mutate(children[0].clone())?);
                next_generation.push(self.mutate(children[1].clone())?);
            }

            population = next_generation;
            let best_combination = population
                .clone()
                .into_iter()
                .max_by(evaluation)
                .ok_or(eyre::eyre!("Best combination not found"))?;
            println!(
                "Generation {} Highest {}: {}",
                generation + 1,
                self.evaluator.target_name,
                self.evaluator.evaluate(best_combination.clone())?
            );
        }

        population.sort_by(evaluation);
        Ok(population.iter().next().unwrap().clone())
    }

    fn generate_random_relic_set(&self) -> Vec<Relic> {
        let mut rng = thread_rng();
        let mut relics = vec![];
        for slot in Slot::iter() {
            let relic = self.relic_pool.get(&slot);
            if let Some(relic) = relic {
                if let Some(relic) = relic.iter().choose(&mut rng) {
                    relics.push(relic.clone());
                }
            }
        }
        relics
    }

    fn crossover(&self, parents: Vec<Vec<Relic>>) -> Result<Vec<Vec<Relic>>> {
        let mut parents = parents.iter();
        let parent1 = parents.next().ok_or(eyre::eyre!("Missing parent 1"))?;
        let parent2 = parents.next().ok_or(eyre::eyre!("Missing parent 2"))?;

        let min_length = min(parent1.len(), parent2.len());

        let mut child1 = vec![];
        let mut child2 = vec![];

        let mut rng = thread_rng();
        for i in 0..min_length {
            if rng.gen::<f64>() > 0.5 {
                child1.push(parent1[i].clone());
                child2.push(parent2[i].clone());
            } else {
                child1.push(parent2[i].clone());
                child2.push(parent1[i].clone());
            }
        }

        if parent1.len() > min_length {
            child1.append(&mut parent1[min_length..].to_vec());
            child2.append(&mut parent1[min_length..].to_vec());
        } else {
            child1.append(&mut parent2[min_length..].to_vec());
            child2.append(&mut parent2[min_length..].to_vec());
        }

        Ok(vec![child1, child2])
    }

    fn mutate(&self, child: Vec<Relic>) -> Result<Vec<Relic>> {
        let mut mutated_child = child.clone();
        let mut rng = thread_rng();

        for i in 0..mutated_child.len() {
            if rng.gen::<f64>() < self.mutation_rate {
                let slot = &mutated_child[i].slot;
                let candidates = self
                    .relic_pool
                    .get(slot)
                    .ok_or(eyre::eyre!("Slot {:?} not found", slot))?;
                let new_relic = candidates
                    .choose(&mut rng)
                    .ok_or(eyre::eyre!("The pool of {:?} is empty", slot))?;
                mutated_child[i] = new_relic.clone();
            }
        }

        Ok(mutated_child)
    }
}
