//! Contains the structs describing the input file
//!

use serde::{Deserialize, Serialize};

/// Description of one simulation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Input {
    pub iterations: Option<usize>,
    pub simulation_type: SimulationType,
}

/// An enum, whose variants correspond to the methods of the [`monty_carlos::MonteCarlo`] struct.
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SimulationType {
    TestStatistic(f64),
    MakeDistribution,
}

/// A batch of simulation inputs
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct InputBatch {
    #[serde(rename = "simulation")]
    batch: Vec<Input>,
}

impl InputBatch {
    /// Creates empty [InputBatch].
    pub fn new() -> Self {
        Self { batch: Vec::new() }
    }

    /// Adds `input` to `self`
    pub fn push(&mut self, input: Input) {
        self.batch.push(input);
    }
}
