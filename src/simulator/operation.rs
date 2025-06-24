//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! operations related module

use crate::simulator::{Instructions, Simulator, SimulatorError};

pub fn operate(sim: &mut Simulator) -> Result<(), SimulatorError> {
  assert!(sim.program_counter < sim.instructions.len()); 
  let instruction = sim.instructions[sim.program_counter].clone();
  match instruction {
    Instructions::LI(a, b) => li_operation(sim, a, b),
    Instructions::MOVE(a, b) => move_operation(sim, a, b),
    Instructions::ADD(a, b, c) => add_operation(sim, a, b, c),
    Instructions::SUB(a, b, c) => sub_operation(sim, a, b, c),
    Instructions::MUL(a, b, c) => mul_operation(sim, a, b, c),
    Instructions::DIV(a, b, c) => div_operation(sim, a, b, c),
    Instructions::REM(a, b, c) => rem_operation(sim, a, b, c),
    Instructions::EXIT => exit_operation(),
    Instructions::SKIP => Ok(()),
    Instructions::JUMP(a) => jump_operation(sim, &a),
    Instructions::PRINT(a) => print_operation(sim, a),
    Instructions::BEQ(a, b, c) => beq_operation(sim, a, b, &c),
    Instructions::BNE(a, b, c) => bne_operation(sim, a, b, &c),
    Instructions::BLT(a, b, c) => blt_operation(sim, a, b, &c),
    Instructions::BLE(a, b, c) => ble_operation(sim, a, b, &c),
    Instructions::BGT(a, b, c) => bgt_operation(sim, a, b, &c),
    Instructions::BGE(a, b, c) => bge_operation(sim, a, b, &c),
  }
}

/// Do the LI instruction operation.
fn li_operation(sim: &mut Simulator, a: usize, b: i32) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    sim.int_registers[a] = b;
    Ok(())
  }
}

/// Do the MOVE instruction operation.
fn move_operation(sim: &mut Simulator, a: usize, b: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    sim.int_registers[a] = sim.int_registers[b];
    Ok(())
  }
}

/// Do the ADD instruction operation.
fn add_operation(sim: &mut Simulator, a: usize, b: usize, c: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() || c >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    let result: i32 = sim.int_registers[b].wrapping_add(sim.int_registers[c]);
    sim.int_registers[a] = result;
    Ok(())
  }
}

/// Do the SUB instruction operation.
fn sub_operation(sim: &mut Simulator, a: usize, b: usize, c: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() || c >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    let result: i32 = sim.int_registers[b].wrapping_sub(sim.int_registers[c]);
    sim.int_registers[a] = result;
    Ok(())
  }
}

/// Do the MUL instruction operation.
fn mul_operation(sim: &mut Simulator, a: usize, b: usize, c: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() || c >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    let result: i32 = sim.int_registers[b].wrapping_mul(sim.int_registers[c]);
    sim.int_registers[a] = result;
    Ok(())
  }
}

/// Do the DIV instruction operation.
fn div_operation(sim: &mut Simulator, a: usize, b: usize, c: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() || c >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[c] == 0 {
    Err(SimulatorError::DivisionByZero)
  } else {
    let result: i32 = sim.int_registers[b].wrapping_div(sim.int_registers[c]);
    sim.int_registers[a] = result;
    Ok(())
  }
}

/// Do the REM instruction operation.
fn rem_operation(sim: &mut Simulator, a: usize, b: usize, c: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() || c >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[c] == 0 {
    Err(SimulatorError::DivisionByZero)
  } else {
    let result: i32 = sim.int_registers[b].wrapping_rem(sim.int_registers[c]);
    sim.int_registers[a] = result;
    Ok(())
  }
}

/// Do the PRINT instruction operation
fn print_operation(sim: &mut Simulator, a: usize) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else {
    println!("PRINT: ${}: {}", a, sim.int_registers[a]);
    Ok(())
  }
}

/// Do the EXIT instruction operation
fn exit_operation() -> Result<(), SimulatorError> {
  println!("EXIT");
  std::process::exit(0);
}

// Do the inconditional JUMP instruction operation
fn jump_operation(sim: &mut Simulator, a: &str) -> Result<(), SimulatorError> {
  let x = sim.labels.get(a);
  if x.is_none() {
    Err(SimulatorError::UnknownLabel)
  } else {
    sim.program_counter = *x.unwrap();
    Ok(())
  }
}

/// Do the conditional BEQ instruction operation
fn beq_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] == sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}

/// Do the conditional BNE instruction operation
fn bne_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] != sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}

/// Do the conditional BLT instruction operation
fn blt_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] < sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}

/// Do the conditional BLE instruction operation
fn ble_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] <= sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}

/// Do the conditional BGT instruction operation
fn bgt_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] > sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}

/// Do the conditional BGE instruction operation
fn bge_operation(sim: &mut Simulator, a: usize, b: usize, c: &str) -> Result<(), SimulatorError> {
  if a >= sim.int_registers.len() || b >= sim.int_registers.len() {
    Err(SimulatorError::OutOfRange)
  } else if sim.int_registers[a] >= sim.int_registers[b] {
    jump_operation(sim, c)
  } else {
    Ok(())
  }
}