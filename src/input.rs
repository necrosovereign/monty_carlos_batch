//! Contains the structs describing the input file
//!

use monty_carlos::MonteCarlo;
use serde::{Deserialize, Serialize};

/// Description of one simulation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Input {
    pub iterations: Option<usize>,
    pub simulation_type: SimulationType,
    pub simulation: simulation::Simulation,
}

impl Input {
    pub(crate) fn run_simulation(&self) {
        let sample = self.simulation.generate_sample();
        let mut monte_carlo = MonteCarlo::new(sample);
        if let Some(iterations) = self.iterations {
            monte_carlo.iterations = iterations;
        }
        match self.simulation_type {
            SimulationType::TestStatistic(test_statistic) => {
                let pvalue = monte_carlo.simulate_pvalue(test_statistic);
                println!("probability = {pvalue}");
            }
            SimulationType::MakeDistribution => {
                let distribution = monte_carlo.simulate_distribution().unwrap();
                println!("percentiles = {distribution:?}");
            }
            SimulationType::Probability(alpha) => {
                let statistic = monte_carlo.simulate_statistic(alpha);
                println!("statistic = {statistic}");
            }
        }
    }
}

/// An enum, whose variants correspond to the methods of the [`monty_carlos::MonteCarlo`] struct.
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SimulationType {
    TestStatistic(f64),
    MakeDistribution,
    Probability(f64),
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
