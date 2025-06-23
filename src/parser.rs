//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use core::fmt;
use regex::Regex;
use std::sync::LazyLock;

use crate::simulator::{Instructions, Simulator};

static INSTRUCTION_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*([A-Z]+)(?:\s+.*)*$").unwrap());
//const INSTRUCTION_PARSER: &str = r"^\s*([A-Z]+)(?:\s+.*)*$";
//static AVOID_PARSER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*(?:\/\/.*)?\s*$").unwrap());
const AVOID_PARSER: &str = r"^\s*(?:\/\/.*)?\s*$";
static LI_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:LI)\s+\$(\d+)\s+(-?\d+)\s*$").unwrap());
//const LI_PARSER: &str = r"^\s*(?:LI)\s+\$(\d+)\s+(-?\d+)\s*$";
static ARITHMETIC_PARSER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^\s*(?:ADD|SUB|MUL|DIV|REM)\s+\$(\d+)\s+\$(\d+)\s+\$(\d+)\s*$").unwrap()
});
//const ARITHMETIC_PARSER: &str = r"^\s*(?:ADD|SUB|MUL|DIV|REM)\s+\$(\d+)\s+\$(\d+)\s+\$(\d+)\s*$";
static PRINT_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:PRINT)\s+\$(\d+)\s*$").unwrap());
//const PRINT_PARSER: &str = r"^\s*(?:PRINT)\s+\$(\d+)\s*$";
static JUMP_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:JUMP)\s+(@[A-Z]+)\s*$").unwrap());
//onst JUMP_PARSER: &str = r"^\s*(?:JUMP)\s+(@[A-Z]+)\s*$";
static COND_JUMP_PARSER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^\s*(?:BEQ|BNE|BLT|BLE|BGT|BGE)\s+\$(\d+)\s+\$(\d+)\s+(@[A-Z]+)\s*$").unwrap()
});
//const COND_JUMP_PARSER: &str = r"^\s*(?:BEQ|BNE|BLT|BLE|BGT|BGE)\s+\$(\d+)\s+\$(\d+)\s+(@[A-Z]+)\s*$";

const LABEL_PARSER: &str = r"^\s*(@[A-Z])\s*$";

#[derive(Debug, PartialEq)]
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
  let regex = Regex::new(AVOID_PARSER).expect("error compiling regex");
  let mut container = Vec::with_capacity(lines.len());
  for n in lines {
    if !regex.is_match(n) {
      container.push(n.to_owned());
    }
  }
  container
}

pub fn process_lines(lines: &[String], sim: &mut Simulator) -> Result<(), ParsingError> {
  let label_parser = Regex::new(LABEL_PARSER).expect("error compiling regex");

  for n in lines.iter().enumerate() {
    if label_parser.is_match(n.1) {
      sim.instructions.push(Instructions::SKIP);
      sim.labels.insert(n.1.to_owned(), n.0);
    }
  }

  Ok(())
}

pub fn parse_instruction(line: &str) -> Result<Instructions, ParsingError> {
  let inst = INSTRUCTION_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidInstruction)?;
  match &inst[1] {
    "LI" => {
      let params = parse_li(line)?;
      Ok(Instructions::LI(params.0, params.1))
    }
    "ADD" => {
      let params = parse_arithmetic(line)?;
      Ok(Instructions::ADD(params.0, params.1, params.2))
    }
    "SUB" => {
      let params = parse_arithmetic(line)?;
      Ok(Instructions::SUB(params.0, params.1, params.2))
    }
    "MUL" => {
      let params = parse_arithmetic(line)?;
      Ok(Instructions::MUL(params.0, params.1, params.2))
    }
    "DIV" => {
      let params = parse_arithmetic(line)?;
      Ok(Instructions::DIV(params.0, params.1, params.2))
    }
    "REM" => {
      let params = parse_arithmetic(line)?;
      Ok(Instructions::REM(params.0, params.1, params.2))
    }
    "PRINT" => {
      let params = parse_print(line)?;
      Ok(Instructions::PRINT(params))
    }
    "JUMP" => {
      let params = parse_jump(line)?;
      Ok(Instructions::JUMP(params))
    }
    "BEQ" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BEQ(param.0, param.1, param.2))
    }
    "BNE" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BNE(param.0, param.1, param.2))
    }
    "BLT" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BLT(param.0, param.1, param.2))
    }
    "BLE" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BLE(param.0, param.1, param.2))
    }
    "BGT" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BGT(param.0, param.1, param.2))
    }
    "BGE" => {
      let param = parser_cond_jump(line)?;
      Ok(Instructions::BGE(param.0, param.1, param.2))
    }
    "SKIP" => Ok(Instructions::SKIP),
    "EXIT" => Ok(Instructions::EXIT),
    _ =>  Err(ParsingError::InvalidInstruction)
  }
}

fn parse_li(line: &str) -> Result<(usize, i32), ParsingError> {
  let capt = LI_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: i32 = capt[2].parse().expect("error parsing");
  Ok((a, b))
}

fn parse_arithmetic(line: &str) -> Result<(usize, usize, usize), ParsingError> {
  let capt = ARITHMETIC_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: usize = capt[2].parse().expect("error parsing");
  let c: usize = capt[3].parse().expect("error parsing");
  Ok((a, b, c))
}

fn parse_print(line: &str) -> Result<usize, ParsingError> {
  let capt = PRINT_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  Ok(a)
}

fn parse_jump(line: &str) -> Result<String, ParsingError> {
  let capt = JUMP_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidParameter)?;
  Ok(capt[1].to_owned())
}

fn parser_cond_jump(line: &str) -> Result<(usize, usize, String), ParsingError> {
  let capt = COND_JUMP_PARSER
    .captures(line)
    .ok_or(ParsingError::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: usize = capt[2].parse().expect("error parsing");
  Ok((a, b, capt[3].to_owned()))
}


#[cfg(test)]
mod parser_errors {
  use crate::{parser::parse_instruction, simulator::Instructions};
  #[test]
  fn parse_li_test() {
    let line: &str = "LI $64 -6";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::LI(64, -6));
  }

  #[test]
  fn parse_arith_test() {
    let line: &str = "ADD $64 $46 $24";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::ADD(64, 46, 24));
  }

  #[test]
  fn parse_incon_test() {
    let line: &str = "JUMP @ENDLOOP";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::JUMP(String::from("@ENDLOOP")));
  }

  #[test]
  fn parse_uncon_test() {
    let line: &str = "  BGE $4 $31 @ENDLOOP";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::BGE(4, 31, String::from("@ENDLOOP")));
  }

  #[test]
  fn parse_print_test() {
    let line: &str = "  PRINT $4";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::PRINT(4));
  }
}