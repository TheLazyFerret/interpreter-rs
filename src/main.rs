//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Main file

pub mod operation;
pub mod parser;
pub mod simulator;

use crate::simulator::Simulator;
use core::fmt;
use std::{collections::HashMap, env, fs, io::Read};

/// Enum representing all the instructions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instructions {
  LI,    // Load imm
  MOVE,  // Move (copy)
  ADD,   // Addition
  SUB,   // Substraction
  MUL,   // Multiplication
  DIV,   // Division
  REM,   // Remainder
  PRINT, // Print
  EXIT,  // Exit
  SKIP,  // Skip (SKIP, comments and empty lines)
  JUMP,  // Jump to a label
}

/// Enum representing all the possible errors during execution
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
  InvalidParameter,
  InvalidInstruction,
  OutOfRange,
  DivisionByZero,
  MainNotFound,
  UnknownLabel,
}

/// Verbose errors
impl fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::InvalidInstruction => f.write_str("the instruction is not implemented yet"),
      Error::InvalidParameter => f.write_str("the parameters are not valid and can't be parsed"),
      Error::DivisionByZero => f.write_str("division by zero"),
      Error::OutOfRange => f.write_str("the reg is out of the ranges"),
      Error::MainNotFound => f.write_str("main label not found"),
      Error::UnknownLabel => f.write_str("trying to jump to a unknown label. Label not found"),
    }
  }
} // impl fmt::Display for Error

/// Returns a vectors with each line to parse.
fn get_instructions(path: &str) -> Vec<String> {
  let mut file = fs::File::open(path).expect("error opening file");
  let mut buffer: String = String::new();
  file
    .read_to_string(&mut buffer)
    .expect("error reading buffer");
  buffer.lines().map(|x| x.to_string()).collect()
}

/// Does the required operation in each type of instruction
#[rustfmt::skip]
fn operate(line: &str, instruction: Instructions, sim: &mut Simulator, labels: &HashMap<String, usize>) -> Result<(), Error> {
  match instruction {
    Instructions::LI => {
      let params = parser::parse_li(line)?;
      operation::load_integer(sim, params)?;
    }
    Instructions::MOVE => {
      let params = parser::parse_move(line)?;
      operation::move_reg(sim, params)?;
    }
    Instructions::ADD => {
      let params = parser::parse_arithmetic(line)?;
      operation::addition(sim, params)?;
    }
    Instructions::SUB => {
      let params = parser::parse_arithmetic(line)?;
      operation::substraction(sim, params)?;
    }
    Instructions::MUL => {
      let params = parser::parse_arithmetic(line)?;
      operation::multiplication(sim, params)?;
    }
    Instructions::DIV => {
      let params = parser::parse_arithmetic(line)?;
      operation::division(sim, params)?;
    }
    Instructions::REM => {
      let params = parser::parse_arithmetic(line)?;
      operation::remain(sim, params)?;
    }
    Instructions::PRINT => {
      let params = parser::parse_print(line)?;
      operation::print_reg(sim, params)?;
    }
    Instructions::EXIT => {
      operation::exit();
    }
    Instructions::SKIP => {}
    Instructions::JUMP => {
      let params = parser::parse_jump(line)?;
      operation::unc_jump(sim, labels, &params)?;
    }
  }
  Ok(())
}

/// Print status for debug.
fn print_status(line: &str, sim: &Simulator) {
  println!("STATUS => IC: {}, TO PARSE: {}", sim.get_ic(), line);
}

fn search_labels(instructions: &[String]) -> HashMap<String, usize> {
  let mut labels: HashMap<String, usize> = HashMap::new();
  for line in instructions.iter().enumerate() {
    let x = parser::parse_label(line.1);
    if x.is_some() {
      let x = x.unwrap();
      labels.insert(x.to_string(), line.0);
    }
  }
  labels
}

/// main loop of the interpreter.
#[rustfmt::skip]
fn main_loop(instructions: &[String], sim: &mut Simulator, labels: &HashMap<String, usize>, debug: bool) -> Result<(), Error> {
  if labels.get("@MAIN").is_none() {
    return Err(Error::MainNotFound);
  } else {
    let x: usize = labels.get("@MAIN").unwrap().to_owned();
    sim.set_ic(x);
  }

  // main loop
  while sim.get_ic() < instructions.len() {
    let line = &instructions[sim.get_ic()];
    if debug {
      print_status(line, sim);
    }
    let instruction = parser::parse_instruction(line)?;
    operate(line, instruction, sim, labels)?;
    sim.set_ic(sim.get_ic() + 1);
  }
  println!("END OF PROGRAM");
  Ok(())
}

/// Main function.
fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("the number of paremets is not correct!");
  }
  let instructions = get_instructions(&args[1]);
  let mut sim = Simulator::new();
  let labels = search_labels(&instructions);
  main_loop(&instructions, &mut sim, &labels, false)?;

  Ok(())
}
