pub mod crossover;
pub mod mutation;

use std::fmt::Display;

use rand::{distributions::Uniform, prelude::Distribution, Rng};

pub trait Individual {
    fn mutate<R: Rng>(self, rate: f64, rng: &mut R) -> Self;
    fn cross<R: Rng>(&self, other: &Self, rng: &mut R) -> (Self, Self)
    where
        Self: Sized;
}

pub trait Problem<I: Individual, O: PartialOrd> {
    fn generate_individual<R: Rng>(&self, rng: &mut R) -> I;
    fn fitness(&self, individual: &I) -> O;
}

pub struct GaParameters {
    pub pop_size: usize,
    pub tour_size: usize,
    pub mutation_prob: f64,
    pub crossover_prob: f64,
}

#[derive(Debug, Clone)]
pub struct IterationResult<I: Individual, O: PartialOrd> {
    pub best: I,
    pub best_fitness: O,
}

pub struct Ga<'a, I, O, P, R> {
    problem: &'a P,
    params: &'a GaParameters,
    current_population: &'a mut Vec<(I, O)>,
    rng: &'a mut R,
}

impl<'a, I: Individual + Clone, O: PartialOrd + Clone, P: Problem<I, O>, R: Rng>
    Ga<'a, I, O, P, R>
{
    pub fn new(
        problem: &'a P,
        params: &'a GaParameters,
        empty_vec: &'a mut Vec<(I, O)>,
        rng: &'a mut R,
    ) -> Ga<'a, I, O, P, R> {
        let init_population = (0..params.pop_size).into_iter().map(|_| {
            let individual = problem.generate_individual(rng);
            let fitness = problem.fitness(&individual);
            (individual, fitness)
        });
        empty_vec.extend(init_population);
        Ga {
            problem,
            params,
            current_population: empty_vec,
            rng,
        }
    }
}

impl<
        'a,
        I: Individual + Clone,
        O: PartialOrd + Display + Clone,
        P: Problem<I, O>,
        R: Rng + Clone,
    > Iterator for Ga<'a, I, O, P, R>
{
    type Item = IterationResult<I, O>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_population: Vec<(I, O)> = next_iteration(
            self.current_population,
            self.params.mutation_prob,
            self.params.tour_size,
            self.params.crossover_prob,
            self.rng,
        )
        .into_iter()
        .map(|i| {
            let o = self.problem.fitness(&i);
            (i, o)
        })
        .collect();
        self.current_population.clear();
        self.current_population.append(&mut new_population);
        Some(create_statistics(self.current_population))
    }
}

fn next_iteration<I: Individual + Clone, O: PartialOrd + Display, R: Rng>(
    current_population: &Vec<(I, O)>,
    mutation_prob: f64,
    tour_size: usize,
    crossover_prob: f64,
    rng: &mut R,
) -> Vec<I> {
    let selection = select(current_population, tour_size, rng);

    let mut pop: Vec<I> = selection.iter().map(|(i, _)| i.clone()).collect();
    for i in (0..pop.len()).step_by(2) {
        if rng.gen::<f64>() < crossover_prob {
            let next_index = (i + 1) % pop.len();
            let (a, b) = pop[i].cross(&pop[next_index], rng);
            pop[i] = a;
            pop[next_index] = b;
        }
    }
    pop.into_iter()
        .map(|i| i.mutate(mutation_prob, rng))
        .collect()
}

fn select<'a, I, O: PartialOrd + Display, R: Rng>(
    current_population: &'a Vec<(I, O)>,
    tour_size: usize,
    rng: &'a mut R,
) -> Vec<&'a (I, O)> {
    let dist = Uniform::from(0..current_population.len());
    (0..current_population.len())
        .into_iter()
        .map(|_| {
            (0..tour_size)
                .into_iter()
                .map(|_| dist.sample(rng))
                .map(|n| &current_population[n])
                .min_by(|(_, fitness1), (_, fitness2)| {
                    fitness1.partial_cmp(fitness2).expect(&format!(
                        "could not compare {}  with {}",
                        fitness1, fitness2
                    ))
                })
                .unwrap()
        })
        .collect()
}

fn create_statistics<I: Individual + Clone, O: PartialOrd + Display + Clone>(
    population: &Vec<(I, O)>,
) -> IterationResult<I, O> {
    let (best, best_fitness) = population
        .into_iter()
        .min_by(|(_, fitness1), (_, fitness2)| {
            fitness1.partial_cmp(fitness2).expect(&format!(
                "could not compare {}  with {}",
                fitness1, fitness2
            ))
        })
        .unwrap()
        .clone();
    IterationResult { best, best_fitness }
}
