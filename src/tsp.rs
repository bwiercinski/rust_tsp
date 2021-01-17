use crate::ga::{crossover, mutation, Individual, Problem};
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct TspCase {
    pub destination_matrix: Vec<Vec<f64>>,
}

impl TspCase {
    pub fn new(cities: &[(f64, f64)]) -> TspCase {
        let destination_matrix = cities
            .iter()
            .map(|row| {
                cities
                    .iter()
                    .map(|column| f64::hypot(column.0 - row.0, column.1 - row.1))
                    .collect()
            })
            .collect();
        TspCase { destination_matrix }
    }
}

impl Problem<TspIndividual, f64> for TspCase {
    fn generate_individual<R: Rng>(&self, rng: &mut R) -> TspIndividual {
        TspIndividual {
            cities: {
                let mut cities: Vec<usize> = (0..self.destination_matrix.len()).collect();
                cities.shuffle(rng);
                cities
            },
        }
    }

    fn fitness(&self, individual: &TspIndividual) -> f64 {
        let mut sum = 0.0;
        for (i, city1) in individual.cities.iter().enumerate() {
            let next_city_idx = (i + 1) % individual.cities.len();
            let city2 = individual.cities[next_city_idx];
            sum += self.destination_matrix[*city1][city2];
        }
        sum
    }
}

#[derive(Debug, Clone)]
pub struct TspIndividual {
    pub cities: Vec<usize>,
}

impl Individual for TspIndividual {
    fn mutate<R: Rng>(self, rate: f64, rng: &mut R) -> Self {
        let mut cities = self.cities;
        mutation::swap_mutation(&mut cities, rate, rng);
        TspIndividual { cities }
    }

    fn cross<R: Rng>(&self, other: &Self, rng: &mut R) -> (Self, Self)
    where
        Self: Sized,
    {
        let (cities1, cities2) = crossover::pmx_crossover(&self.cities, &other.cities, rng);
        (
            TspIndividual { cities: cities1 },
            TspIndividual { cities: cities2 },
        )
    }
}
