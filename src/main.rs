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
mod snowfall;

use input::InputBatch;

fn main() {
    let input_path = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(input_path).unwrap();
    let input_batch: InputBatch = match toml::from_str(&input) {
        Ok(input) => input,
        Err(err) => {
            println!("{err}");
            panic!();
        }
    };
    for input in input_batch.batch {
        input.run_simulation();
    }
}
