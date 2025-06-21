//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use crate::Instructions;
use core::fmt;
use regex::Regex;

const INSTRUCTION_REGEX: &str = r"^\s*([A-Z]+)(\s+\S+)*\s*$";
const LI_REGEX: &str = r"^\s*(LI)\s+(\d+)\s+(\-?\d+)\s*$";
const MOVE_REGEX: &str = r"^\s*(MOVE)\s+(\d+)\s+(\d+)\s*$";
const ARITHMETIC_REGEX: &str = r"^\s*(ADD|SUB|MUL|DIV|REM)\s+(\d+)\s+(\d+)\s+(\d+)\s*$";
const PRINT_REGEX: &str = r"^\s*(PRINT)\s+(\d+)\s*$";
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
} // impl fmt::Display for ParseError

/// Returns the type of instruction is in the line. Returns ParseError::InvalidInstruction if it fail
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

/// Returns the three parameters of the arimenthic instructions (REG0, REG1, REG2). Returns ParseError::InvalidParameter if it fail
pub fn parse_arithmetic(line: &str) -> Result<(usize, usize, usize), ParseError> {
  let regex = Regex::new(ARITHMETIC_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(ParseError::InvalidParameter);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: usize = capture[3].parse().expect("error parsing");
  let z: usize = capture[4].parse().expect("error parsing");
  Ok((x, y, z))
}

/// Returns the two paremeters of the LI instruction (REG0, INMM). Returns ParseError::InvalidParameter if it fail
pub fn parse_li(line: &str) -> Result<(usize, i32), ParseError> {
  let regex = Regex::new(LI_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(ParseError::InvalidInstruction);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: i32 = capture[3].parse().expect("error parsing");
  Ok((x, y))
}

/// Returns the two paremeters of the MOVE instruction (REG0, REG1). Returns ParseError::InvalidParameter if it fail
pub fn parse_move(line: &str) -> Result<(usize, usize), ParseError> {
  let regex = Regex::new(MOVE_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(ParseError::InvalidInstruction);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: usize = capture[3].parse().expect("error parsing");
  Ok((x, y))
}

/// Returns the single paremeter of the PRINT instruction (REG). Returns ParseError::InvalidParameter if it fail
pub fn parse_print(line: &str) -> Result<usize, ParseError> {
  let regex = Regex::new(PRINT_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(ParseError::InvalidInstruction);
  }
  let capture = capture.unwrap();
  Ok(capture[2].parse().expect("error parsing"))
}

#[cfg(test)]
mod parser_test {
  use crate::parser::{parse_arithmetic, parse_instruction};

  #[test]
  fn parse_instruction_test() {
    let line0: &str = "MUL 5 10 25";
    let line1: &str = "LI 5 -10";
    let line2: &str = "EXIT";
    let line3: &str = "SUB 39 102 9";
    parse_instruction(line0).expect("error found");
    parse_instruction(line1).expect("error found");
    parse_instruction(line2).expect("error found");
    parse_instruction(line3).expect("error found");
  }

  #[test]
  fn parse_arithmetic_test() {
    let line0: &str = "MUL 5 10 25";
    let line1: &str = "SUB 39 102 9";
    let line2: &str = "LI 5 -10";
    let x = parse_arithmetic(line0).expect("unexpected error");
    let y = parse_arithmetic(line1).expect("unexpected error");
    parse_arithmetic(line2).expect_err("unexpected sucess");
    assert_eq!(x, (5, 10, 25));
    assert_eq!(y, (39, 102, 9));
  }
}
