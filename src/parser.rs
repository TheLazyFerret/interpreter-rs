//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use crate::Instructions;
use core::fmt;
use regex::Regex;

const INSTRUCTION_REGEX: &str = r"^\s*([A-Z]+)(\s+\S+)*\s*$";
const _LI_REGEX: &str = r"^\s*(LI)\s+(\d+)\s+(\-?\d+)\s*$";
const _MOVE_REGEX: &str = r"^\s*(MOVE)\s+(\d+)\s+(\d+)\s*$";
const _ARITHMETIC_REGEX: &str = r"^\s*(ADD|SUB|MUL|DIV|REM)\s+(\d+)\s+(\d+)\s+(\d+)\s*$";
const _PRINT_REGEX: &str = r"^\s*(PRINT)\s+(\d+)\s*$";
const _EXIT_REGEX: &str = r"^\s*(EXIT)\s*$";

/// Enum representing parsing errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseError {
  InvalidParameter,
  InvalidInstruction,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseError::InvalidInstruction => f.write_str("the instruction is not implemented yet"),
      ParseError::InvalidParameter => {
        f.write_str("the parameters are not valid and can't be parsed")
      }
    }
  }
}

pub fn parse_instruction(line: &str) -> Result<Instructions, ParseError> {
  let regex = Regex::new(INSTRUCTION_REGEX).expect("error compiling the regular expresion");
  if !regex.is_match(line) {
    return Err(ParseError::InvalidInstruction);
  }
  let capture = regex.captures(line).unwrap();
  match &capture[1] {
    "LI" => Ok(Instructions::LI),
    "MOVE " => Ok(Instructions::MOVE),
    "ADD" => Ok(Instructions::ADD),
    "SUB" => Ok(Instructions::SUB),
    "MUL" => Ok(Instructions::MUL),
    "DIV" => Ok(Instructions::DIV),
    "REM" => Ok(Instructions::REM),
    "PRINT" => Ok(Instructions::PRINT),
    "EXIT" => Ok(Instructions::EXIT),
    _ => Err(ParseError::InvalidInstruction),
  }
}

#[cfg(test)]
mod parser_test {
  use crate::parser::parse_instruction;

  #[test]
  fn parse_instruction_test() {
    let line0 : &str = "MUL 5 10 25";
    let line1 : &str = "LI 5 -10";
    let line2 : &str = "EXIT";
    let line3 : &str = "SUB 39 102 9";
    parse_instruction(line0).expect("error found");
    parse_instruction(line1).expect("error found");
    parse_instruction(line2).expect("error found");
    parse_instruction(line3).expect("error found");
  }
}