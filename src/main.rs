//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main file

pub mod operation;
pub mod parser;
pub mod simulator;

/// Enum representing all the instructions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instructions {
  LI,
  MOVE,
  ADD,
  SUB,
  MUL,
  DIV,
  REM,
  PRINT,
  EXIT,
}

fn main() {
  todo!()
}
