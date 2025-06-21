//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Simulator related module

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Simulator {
  int_registers: [i32; 32],
  instruction_counter: usize,
}

impl Default for Simulator {
  fn default() -> Self {
    Self::new()
  }
} // impl Default for Simulator

impl Simulator {
  /// Creates a new instance of the simulator.
  pub fn new() -> Self {
    Simulator {
      int_registers: [0; 32],
      instruction_counter: 0,
    }
  }

  /// Sets a value to one int register.
  pub fn set_int_reg(&mut self, reg: usize, value: i32) {
    assert!(reg < 32);
    if reg != 0 {
      self.int_registers[reg] = value;
    }
  }

  /// Returns the value of one int register.
  pub fn get_int_reg(&self, reg: usize) -> i32 {
    assert!(reg < 32);
    self.int_registers[reg]
  }

  /// sets program counter
  pub fn set_ic(&mut self, value: usize) {
    self.instruction_counter = value;
  }

  /// get program counter
  pub fn get_ic(&self) -> usize {
    self.instruction_counter
  }
} // impl Simulator
