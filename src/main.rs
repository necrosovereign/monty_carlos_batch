// Copyright 2024 Vladimir Kharchev

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A binary executable that runs Monte-Carlo simulations described in a text file.
//!
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

mod input;

use input::{Input, InputBatch, SimulationType};

fn main() {
    let mut input_batch = InputBatch::new();
    input_batch.push(Input {
        iterations: Some(1000),
        simulation_type: SimulationType::TestStatistic(0.45),
        simulation: input::simulation::Simulation::KSTest {
            samples: 259,
            distribution: input::simulation::Distribution::Normal {
                mean: 0.0,
                stdev: 1.0,
            },
        },
    });
    std::fs::write("test.toml", toml::to_string(&input_batch).unwrap()).unwrap();

    let input: InputBatch = match toml::from_str(&std::fs::read_to_string("example.toml").unwrap())
    {
        Ok(input) => input,
        Err(err) => panic!("{}", err.message()),
    };
    println!("{input:?}");
}
