//! Contains the structs describing the input file
//!

use serde::{Deserialize, Serialize};

/// Description of one simulation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Input {
    pub iterations: Option<usize>,
    pub simulation_type: SimulationType,
    pub simulation: Simulation,
}

/// An enum, whose variants correspond to the methods of the [`monty_carlos::MonteCarlo`] struct.
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SimulationType {
    TestStatistic(f64),
    MakeDistribution,
}

/// An enum that describes possible simulation that can be executed
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub(crate) enum Simulation {
    #[serde(rename = "Kolmogorov_Smirnov_test")]
    KSTest {
        samples: usize,
        distribution: Distribution,
    },
    #[serde(rename = "Lilliefors_test")]
    LillieforsTest {
        samples: usize,
        generating_distribution: Distribution,
        distribution_to_compare: Fit,
    },
}

/// An enum of possible distributions for Kolmogorov-Smirnov and Lilliefors tests
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum Distribution {
    Normal { mean: f64, stdev: f64 },
}

/// An enum of possible distribution kinds that Lilliefors test will fit to generated datasets
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub(crate) enum Fit {
    Normal,
}

/// A batch of simulation inputs
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct InputBatch {
    #[serde(rename = "simulations")]
    batch: Vec<Input>,
}

impl InputBatch {
    /// Creates empty [`InputBatch`].
    pub fn new() -> Self {
        Self { batch: Vec::new() }
    }

    /// Adds `input` to `self`
    pub fn push(&mut self, input: Input) {
        self.batch.push(input);
    }
}
