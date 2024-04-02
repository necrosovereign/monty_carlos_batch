//! Contains the structs describing the input file
//!

use monty_carlos::MonteCarlo;
use serde::{Deserialize, Serialize};

/// Description of one simulation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Input {
    pub iterations: Option<usize>,
    pub queries: Box<[Query]>,
    pub simulation: simulation::Simulation,
}

impl Input {
    pub(crate) fn run_simulation(&self) {
        let sample = self.simulation.generate_sample();
        let mut monte_carlo = MonteCarlo::new(sample);
        if let Some(iterations) = self.iterations {
            monte_carlo.iterations = iterations;
        }
        if let [Query::TestStatistic(test_statistic)] = *self.queries {
            let pvalue = monte_carlo.simulate_pvalue(test_statistic);
            println!("probability = {pvalue}");
            return;
        }
        let distribution = monte_carlo.simulate();
        for query in self.queries.iter() {
            match query {
                Query::TestStatistic(test_statistic) => {
                    let pvalue = distribution.pvalue(*test_statistic);
                    println!("probability = {pvalue}");
                }
                Query::MakeDistribution => {
                    let quantiles = distribution.quantiles();
                    println!("percentiles = {quantiles:?}");
                }
                Query::Probability(alpha) => {
                    let statistic = distribution.quantile_of(*alpha);
                    println!("statistic = {statistic}");
                }
                Query::MeanAndStdev => {
                    let mean = distribution.mean();
                    let stdev = distribution.stdev();
                    println!("mean = {mean}");
                    println!("stdev = {stdev}");
                }
            }
        }
    }
}

/// An enum, whose variants correspond to the methods of the [`monty_carlos::MonteCarlo`] struct.
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Query {
    TestStatistic(f64),
    MakeDistribution,
    Probability(f64),
    MeanAndStdev,
}

pub mod simulation;

/// A batch of simulation inputs
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct InputBatch {
    #[serde(rename = "simulations")]
    pub batch: Vec<Input>,
}

// impl InputBatch {
//     /// Creates empty [`InputBatch`].
//     pub fn new() -> Self {
//         Self { batch: Vec::new() }
//     }

//     /// Adds `input` to `self`
//     pub fn push(&mut self, input: Input) {
//         self.batch.push(input);
//     }
// }
