//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Simulator related module

use crate::simulator::parser::{preprocess_lines, process_lines};
use std::{collections::HashMap, fmt};

pub mod operation;
pub mod parser;

/// Struct representing the machine.
#[derive(Debug, PartialEq, Default)]
pub struct Simulator {
  int_registers: [i32; 32],
  program_counter: usize,
  labels: HashMap<String, usize>,
  instructions: Vec<Instructions>,
}

/// Enum representing all the instructions.
#[derive(Debug, PartialEq, Clone)]
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
  LABEL,
  JUMP(String),              // Jump to a label
  BEQ(usize, usize, String), // Jump to label if a == b
  BNE(usize, usize, String), // Jump to label if a != b
  BLT(usize, usize, String), // Jump to label if a < b
  BLE(usize, usize, String), // Jump to label if a <= b
  BGT(usize, usize, String), // Jump to label if a > b
  BGE(usize, usize, String), // Jump to label if a >= b
}

impl fmt::Display for Instructions {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let instruction = self.clone();
    match instruction {
      Instructions::LI(a, b) => write!(f, "LI ${a} {b}"),
      Instructions::MOVE(a, b) => write!(f, "MOVE ${a} ${b}"),
      Instructions::ADD(a, b, c) => write!(f, "ADD ${a} ${b} ${c}"),
      Instructions::SUB(a, b, c) => write!(f, "SUB ${a} ${b} ${c}"),
      Instructions::MUL(a, b, c) => write!(f, "MUL ${a} ${b} ${c}"),
      Instructions::DIV(a, b, c) => write!(f, "DIV ${a} ${b} ${c}"),
      Instructions::REM(a, b, c) => write!(f, "REM ${a} ${b} ${c}"),
      Instructions::PRINT(a) => write!(f, "PRINT ${a}"),
      Instructions::EXIT => write!(f, "EXIT"),
      Instructions::SKIP => write!(f, "SKIP"),
      Instructions::JUMP(a) => write!(f, "JUMP @{}", &a),
      Instructions::BEQ(a, b, c) => write!(f, "BEQ ${a} ${b} {}", &c),
      Instructions::BNE(a, b, c) => write!(f, "BNE ${a} ${b} {}", &c),
      Instructions::BLT(a, b, c) => write!(f, "BLT ${a} ${b} {}", &c),
      Instructions::BLE(a, b, c) => write!(f, "BLE ${a} ${b} {}", &c),
      Instructions::BGT(a, b, c) => write!(f, "BGT ${a} ${b} {}", &c),
      Instructions::BGE(a, b, c) => write!(f, "BGE ${a} ${b} {}", &c),
      Instructions::LABEL => write!(f, "LABEL")
    }
  }
}

/// Enum representing all the possible errors during runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
  OutOfRange,
  DivisionByZero,
  MainNotFound,
  UnknownLabel,
  InvalidInstruction,
  InvalidParameter,
}

/// trait for verbose errors.
impl fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::DivisionByZero => f.write_str("division by zero"),
      Error::OutOfRange => f.write_str("the reg is out of the ranges"),
      Error::MainNotFound => f.write_str("main label not found"),
      Error::UnknownLabel => {
        f.write_str("trying to jump to a unknown label. Label not found")
      },
      Error::InvalidInstruction => f.write_str("the instruction is not valid, or doesn't exist"),
      Error::InvalidParameter => f.write_str("the parameters are not valid")

    }
  }
} // impl fmt::Display for Error

impl Simulator {
  /// Creates
  pub fn new() -> Self {
    Simulator::default()
  }

  fn step(&mut self, debug: bool) -> Result<(), Error> {
    if debug {
      println!("{}", self.instructions[self.program_counter]);
    }
    operation::operate(self)?;
    self.program_counter += 1;
    Ok(())
  }

  pub fn load(&mut self, raw_lines: &[String]) -> Result<(), Error> {
    print!("Preprocess...");
    let preprocess = preprocess_lines(raw_lines);
    println!(" Done");
    
    print!("Parsing...");
    process_lines(&preprocess, self)?;
    println!(" Done");
    Ok(())
  }

  pub fn run(&mut self, debug: bool) -> Result<(), Error> {
    self.program_counter = self
      .labels
      .get("@MAIN")
      .ok_or(Error::MainNotFound)?
      .clone();
    while self.program_counter < self.instructions.len() {
      self.step(debug)?;
    }

    todo!();
  }
} // impl Simulator
