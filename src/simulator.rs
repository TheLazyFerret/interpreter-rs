//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Simulator related module

use std::{collections::HashMap, fmt};

/// Struct representing the machine.
#[derive(Debug, PartialEq, Default)]
pub struct Simulator {
  pub int_registers: [i32; 32],
  pub program_counter: usize,
  pub labels: HashMap<String, usize>,
  pub instructions: Vec<Instructions>,
}

impl Simulator {
  pub fn new() -> Self {
    Simulator::default()
  }
}

/// Enum representing all the instructions.
#[derive(Debug, PartialEq)]
pub enum Instructions {
  LI(usize, i32),            // Load imm
  MOVE(usize, usize),        // Move (copy)
  ADD(usize, usize, usize),  // Addition
  SUB(usize, usize, usize),  // Substraction
  MUL(usize, usize, usize),  // Multiplication
  DIV(usize, usize, usize),  // Division
  REM(usize, usize, usize),  // Remainder
  PRINT(usize),              // Print
  EXIT,                      // Exit
  SKIP,                      // Skip the line (no operation)
  JUMP(String),              // Jump to a label
  BEQ(usize, usize, String), // Jump to label if a == b
  BNE(usize, usize, String), // Jump to label if a != b
  BLT(usize, usize, String), // Jump to label if a < b
  BLE(usize, usize, String), // Jump to label if a <= b
  BGT(usize, usize, String), // Jump to label if a > b
  BGE(usize, usize, String), // Jump to label if a >= b
}

/// Enum representing all the possible errors during runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimulatorError {
  OutOfRange,
  DivisionByZero,
  MainNotFound,
  UnknownLabel,
}

/// trait for verbose errors.
impl fmt::Display for SimulatorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SimulatorError::DivisionByZero => f.write_str("division by zero"),
      SimulatorError::OutOfRange => f.write_str("the reg is out of the ranges"),
      SimulatorError::MainNotFound => f.write_str("main label not found"),
      SimulatorError::UnknownLabel => {
        f.write_str("trying to jump to a unknown label. Label not found")
      }
    }
  }
} // impl fmt::Display for Error
