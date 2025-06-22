//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use crate::{Error, Instructions};
use regex::Regex;
use std::sync::LazyLock;

static INSTRUCTION_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*([A-Z]+)(\s+\S+)*\s*$").unwrap());
const LI_REGEX: &str = r"^\s*(LI)\s+\$(\d+)\s+(\-?\d+)\s*$";
const MOVE_REGEX: &str = r"^\s*(MOVE)\s+\$(\d+)\s+\$(\d+)\s*$";
const ARITHMETIC_REGEX: &str = r"^\s*(?:ADD|SUB|MUL|DIV|REM)\s+\$(\d+)\s+\$(\d+)\s+\$(\d+)\s*$";
const PRINT_REGEX: &str = r"^\s*(PRINT)\s+\$(\d+)\s*$";
const SKIP_REGEX: &str = r"^(\s*|\/\/.*|SKIP.*)$";
const LABEL_REGEX: &str = r"^\s*(@[A-Z]+)\s*$";
const JUMP_REGEX: &str = r"^\s*(JUMP)\s+(@[A-Z]+)\s*$";
const CONDITIONAL_JUMP_REGEX: &str =
  r"^\s*(BEQ|BNE|BLT|BLE|BGT|BGE)\s+\$(\d+)\s+\$(\d+)\s+(@[A-Z]+)\s*$";

/// Returns the type of instruction is in the line. Returns Error::InvalidInstruction if it fail.
pub fn parse_instruction(line: &str) -> Result<Instructions, Error> {
  let skip = Regex::new(SKIP_REGEX).expect("error compiling the regular expresion");
  let label = Regex::new(LABEL_REGEX).expect("error compiling the regular expresion");
  if skip.is_match(line) || label.is_match(line) {
    return Ok(Instructions::SKIP);
  }

  let capture = INSTRUCTION_REGEX
    .captures(line)
    .ok_or(Error::InvalidInstruction)?;
  match &capture[1] {
    "LI" => Ok(Instructions::LI),
    "MOVE" => Ok(Instructions::MOVE),
    "ADD" => Ok(Instructions::ADD),
    "SUB" => Ok(Instructions::SUB),
    "MUL" => Ok(Instructions::MUL),
    "DIV" => Ok(Instructions::DIV),
    "REM" => Ok(Instructions::REM),
    "PRINT" => Ok(Instructions::PRINT),
    "EXIT" => Ok(Instructions::EXIT),
    "JUMP" => Ok(Instructions::JUMP),
    "BEQ" => Ok(Instructions::BEQ),
    "BNE" => Ok(Instructions::BNE),
    "BLT" => Ok(Instructions::BLT),
    "BLE" => Ok(Instructions::BLE),
    "BGT" => Ok(Instructions::BGT),
    "BGE" => Ok(Instructions::BGE),
    _ => Err(Error::InvalidInstruction),
  }
}

/// Returns the three parameters of the arimenthic instructions (REG0, REG1, REG2). Returns Error::InvalidParameter if it fail.
pub fn parse_arithmetic(line: &str) -> Result<(usize, usize, usize), Error> {
  let regex = Regex::new(ARITHMETIC_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(Error::InvalidParameter);
  }
  let capture = capture.unwrap();
  let x: usize = capture[1].parse().expect("error parsing");
  let y: usize = capture[2].parse().expect("error parsing");
  let z: usize = capture[3].parse().expect("error parsing");
  Ok((x, y, z))
}

/// Returns the two paremeters of the LI instruction (REG0, imm). Returns Error::InvalidParameter if it fail.
pub fn parse_li(line: &str) -> Result<(usize, i32), Error> {
  let regex = Regex::new(LI_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(Error::InvalidInstruction);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: i32 = capture[3].parse().expect("error parsing");
  Ok((x, y))
}

/// Returns the two paremeters of the MOVE instruction (REG0, REG1). Returns Error::InvalidParameter if it fail.
pub fn parse_move(line: &str) -> Result<(usize, usize), Error> {
  let regex = Regex::new(MOVE_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(Error::InvalidInstruction);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: usize = capture[3].parse().expect("error parsing");
  Ok((x, y))
}

/// Returns the single paremeter of the PRINT instruction (REG). Returns Error::InvalidParameter if it fail.
pub fn parse_print(line: &str) -> Result<usize, Error> {
  let regex = Regex::new(PRINT_REGEX).expect("error compilating the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(Error::InvalidInstruction);
  }
  let capture = capture.unwrap();
  Ok(capture[2].parse().expect("error parsing"))
}

/// Returns Some if the instruction is a label, None otherwise
pub fn parse_label(line: &str) -> Option<String> {
  let regex = Regex::new(LABEL_REGEX).expect("error compiling the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    None
  } else {
    let capture = capture.unwrap();
    Some(capture[1].to_string())
  }
}

/// Returns the single parameters of the JUMP instruction (LABEL). Returns Error::InvalidParameter if it fail.
pub fn parse_jump(line: &str) -> Result<String, Error> {
  let regex = Regex::new(JUMP_REGEX).expect("error compiling the regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    Err(Error::InvalidParameter)
  } else {
    let capture = capture.unwrap();
    Ok(capture[2].to_string())
  }
}

/// Returns the parameters of the conditional jump instructions (BXX). Returns Error::InvalidParameter if it fail.
pub fn parse_cond_jump(line: &str) -> Result<(usize, usize, String), Error> {
  let regex = Regex::new(CONDITIONAL_JUMP_REGEX).expect("error compiling regular expresion");
  let capture = regex.captures(line);
  if capture.is_none() {
    return Err(Error::InvalidInstruction);
  }
  let capture = capture.unwrap();
  let x: usize = capture[2].parse().expect("error parsing");
  let y: usize = capture[3].parse().expect("error parsing");
  Ok((x, y, capture[4].to_string()))
}

#[cfg(test)]
mod parser_test {

  use crate::parser::{parse_arithmetic, parse_instruction, parse_label};

  #[test]
  fn parse_instruction_test() {
    let line0: &str = "MUL $5 $10 $25";
    let line1: &str = "LI $5 -10";
    let line2: &str = "EXIT";
    let line3: &str = "SUB $39 $102 $9";
    parse_instruction(line0).expect("error found");
    parse_instruction(line1).expect("error found");
    parse_instruction(line2).expect("error found");
    parse_instruction(line3).expect("error found");
  }

  #[test]
  fn parse_arithmetic_test() {
    let line0: &str = "MUL $5 $10 $25";
    let line1: &str = "SUB $39 $102 $9";
    let line2: &str = "LI $5 -10";
    let x = parse_arithmetic(line0).expect("unexpected error");
    let y = parse_arithmetic(line1).expect("unexpected error");
    parse_arithmetic(line2).expect_err("unexpected sucess");
    assert_eq!(x, (5, 10, 25));
    assert_eq!(y, (39, 102, 9));
  }

  #[test]
  fn parse_label_test() {
    let line0: &str = "  @AVERYNICELABEL";
    let line1: &str = "@ a_bad_label";
    let line2: &str = "SUM";
    parse_label(line0).expect("unexpected error");
    assert_eq!(parse_label(line1), None);
    assert_eq!(parse_label(line2), None);
  }
} // mod parser_arithmetic_test
