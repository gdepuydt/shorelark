#![feature(type_alias_impl_trait)]
#![feature(crate_visibility_modifier)]

pub use self::{crossover::*, genome::*, individual::*, mutation::*, selection::*, statistics::*};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod crossover;
mod genome;
mod individual;
mod mutation;
mod selection;
mod statistics;

pub struct GeneticAlgorithm<S> {
    crossover_policy: Box<dyn CrossoverPolicy>,
    crossover_probability: f32,
    mutation_policy: Box<dyn MutationPolicy>,
    mutation_probability: f32,
    selection_policy: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionPolicy,
{
    pub fn new(
        crossover_policy: impl CrossoverPolicy + 'static,
        crossover_probability: f32,
        mutation_policy: impl MutationPolicy + 'static,
        mutation_probability: f32,
        selection_policy: S,
    ) -> Self {
        Self {
            crossover_policy: Box::new(crossover_policy),
            crossover_probability,
            mutation_policy: Box::new(mutation_policy),
            mutation_probability,
            selection_policy,
        }
    }

    // TODO missing tests
    pub fn evolve<I>(&self, population: &[I], rng: &mut dyn RngCore) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut new_population = Vec::with_capacity(population.len());

        while new_population.len() < population.len() {
            let mut child_a = self.selection_policy.select(&population, rng).genome();
            let mut child_b = self.selection_policy.select(&population, rng).genome();

            if rng.gen_bool(self.crossover_probability as _) {
                self.crossover_policy
                    .crossover(&mut child_a, &mut child_b, rng);
            }

            for child in [&mut child_a, &mut child_b].iter_mut() {
                if rng.gen_bool(self.mutation_probability as _) {
                    self.mutation_policy.mutate(child, rng);
                }
            }

            new_population.push(I::create(child_a));
            new_population.push(I::create(child_b));
        }

        (new_population, Statistics::new(population))
    }
}
