//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main file

pub mod simulator;

use std::{env, fs::read_to_string};

use crate::simulator::{Simulator, Error};

fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let mut sim = Simulator::new();
  let lines: Vec<String> = read_to_string(&args[1])
    .expect("error reading files")
    .lines()
    .map(|x| x.to_string())
    .collect();
  sim.load(&lines)?;
  sim.run(false)?;
  Ok(())
}
