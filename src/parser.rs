//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use core::fmt;
use regex::Regex;

use crate::simulator::Instructions;

const INSTRUCTION_PARSER: &str = r"^\s*([A-Z]+)(?:\s+.*)*$";
const AVOID_PARSER: &str = r"^\s*(?:\/\/.*)?\s*$";
const LI_PARSER: &str = r"^\s*(?:LI)\s+\$(\d+)\s+(-?\d+)\s*$";
const ARITHMETIC_PARSER: &str = r"^\s*(?:ADD|SUB|MUL|DIV|REM)\s+\$(\d+)\s+\$(\d+)\s+\$(\d+)\s*$";
const PRINT_PARSER: &str = r"^\s*(?:PRINT)\s+\$(\d+)\s*$";
const JUMP_PARSER: &str = r"^\s*(?:JUMP)\s+(@[A-Z]+)\s*$";
const COND_JUMP_PARSER: &str = r"^\s*(?:BEQ|BNE|BLT|BLE|BGT|BGE)\s+\$(\d+)\s+\$(\d+)\s+(@[A-Z]+)\s*$";

pub enum ParsingError {
  InvalidInstruction,
  InvalidParameter,
}

impl fmt::Display for ParsingError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParsingError::InvalidInstruction => {
        f.write_str("the instruction is not valid, or doesn't exist")
      }
      ParsingError::InvalidParameter => f.write_str("the parameters are not valid"),
    }
  }
}

/// Returns a new Vec<String> with all comments and empty lines removed
pub fn preprocess_lines(lines: &[String]) -> Vec<String> {
  let avoid = Regex::new(AVOID_PARSER).expect("error compiling regex");
  let mut processes: Vec<String> = Vec::with_capacity(lines.len());
  for n in lines {
    if !avoid.is_match(n) {
      processes.push(n.to_owned());
    }
  };
  processes
  // alternative way to do, 
  //lines.iter().map(|line: &String|avoid.captures(line)).collect()
}

pub fn process_lines(lines: &[String]) -> Vec<Instructions> {
  todo!()
}