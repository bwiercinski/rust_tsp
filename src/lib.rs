#![feature(test)]

extern crate test;

pub mod ga;
pub mod tsp;

pub use ga::*;
pub use tsp::*;
use rand::*;


pub fn run_ga() {
    let tsp = TspCase::new(&[
        // Oliver 30. The shortest cycle length is 423.741.
        (54.0, 67.0),
        (54.0, 62.0),
        (37.0, 84.0),
        (41.0, 94.0),
        (2.0, 99.0),
        (7.0, 64.0),
        (25.0, 62.0),
        (22.0, 60.0),
        (18.0, 54.0),
        (4.0, 50.0),
        (13.0, 40.0),
        (18.0, 40.0),
        (24.0, 42.0),
        (25.0, 38.0),
        (44.0, 35.0),
        (41.0, 26.0),
        (45.0, 21.0),
        (58.0, 35.0),
        (62.0, 32.0),
        (82.0, 7.0),
        (91.0, 38.0),
        (83.0, 46.0),
        (71.0, 44.0),
        (64.0, 60.0),
        (68.0, 58.0),
        (83.0, 69.0),
        (87.0, 76.0),
        (74.0, 78.0),
        (71.0, 71.0),
        (58.0, 69.0),
    ]);
    let mut rng = thread_rng();

    let params = GaParameters {
        pop_size: 500,
        tour_size: 6,
        mutation_prob: 0.05,
        crossover_prob: 0.6,
    };
    let mut empty_vec = vec![];
    Ga::new(&tsp, &params, &mut empty_vec, &mut rng)
        .into_iter()
        .take(200)
        .min_by(|i, j| i.best_fitness.partial_cmp(&j.best_fitness).unwrap())
        .map(|x| {
            println!("final {:?}", x);
            x
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_run_ga(b: &mut Bencher) {
        b.iter(|| run_ga());
    }
}
