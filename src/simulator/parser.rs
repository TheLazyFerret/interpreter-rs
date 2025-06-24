//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! Parsing related module

use regex::Regex;
use std::sync::LazyLock;

use crate::simulator::{Instructions, Simulator, Error};

static INSTRUCTION_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*([A-Z]+)(?:\s+.*)*$").unwrap());
static LI_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:LI)\s+\$(\d+)\s+(-?\d+)\s*$").unwrap());
static MOVE_PARSER: LazyLock<Regex> = 
  LazyLock::new(|| Regex::new(r"^\s*(?:MOVE)\s+\$(\d+)\s+\$(\d+)\s*$").unwrap());
static ARITHMETIC_PARSER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^\s*(?:ADD|SUB|MUL|DIV|REM)\s+\$(\d+)\s+\$(\d+)\s+\$(\d+)\s*$").unwrap()
});
static PRINT_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:PRINT)\s+\$(\d+)\s*$").unwrap());
static JUMP_PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\s*(?:JUMP)\s+(@[A-Z]+)\s*$").unwrap());
static COND_JUMP_PARSER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^\s*(?:BEQ|BNE|BLT|BLE|BGT|BGE)\s+\$(\d+)\s+\$(\d+)\s+(@[A-Z]+)\s*$").unwrap()
});

const AVOID_PARSER: &str = r"^\s*(?:\/\/.*)?\s*$";
const LABEL_PARSER: &str = r"^\s*@([A-Z]+)\s*$";

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

/// Parse and fill the sim.instructions and sim.labels
pub fn process_lines(lines: &[String], sim: &mut Simulator) -> Result<(), Error> {
  let label_parser = Regex::new(LABEL_PARSER).expect("error compiling regex");
  for n in lines.iter().enumerate() {
    if label_parser.is_match(n.1) {
      sim.instructions.push(Instructions::LABEL);
      sim.labels.insert(n.1.to_owned(), n.0);
    }
    else {
      sim.instructions.push(parse_instruction(n.1)?);
    }
  }
  Ok(())
}

/// Parse each instruction, returning a Instruction or the type of Error
pub fn parse_instruction(line: &str) -> Result<Instructions, Error> {
  let inst = INSTRUCTION_PARSER
    .captures(line)
    .ok_or(Error::InvalidInstruction)?;
  match &inst[1] {
    "LI" => {
      let params = parse_li(line)?;
      Ok(Instructions::LI(params.0, params.1))
    }
    "MOVE" => {
      let param = parse_move(line)?;
      Ok(Instructions::MOVE(param.0, param.1))
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
    _ => Err(Error::InvalidInstruction),
  }
} // fn parse_instruction

/// Parse a LI instruction.
fn parse_li(line: &str) -> Result<(usize, i32), Error> {
  let capt = LI_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: i32 = capt[2].parse().expect("error parsing");
  Ok((a, b))
}

/// Parse a MOVE instruction
fn parse_move(line: &str) -> Result<(usize, usize), Error> {
  let capt = MOVE_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: usize = capt[2].parse().expect("error parsing");
  Ok((a, b))
}

/// Parse a arithmetic (ADD, SUB, MUL, DIV, REM) instruction.
fn parse_arithmetic(line: &str) -> Result<(usize, usize, usize), Error> {
  let capt = ARITHMETIC_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: usize = capt[2].parse().expect("error parsing");
  let c: usize = capt[3].parse().expect("error parsing");
  Ok((a, b, c))
}

/// Parse a PRINT instruction.
fn parse_print(line: &str) -> Result<usize, Error> {
  let capt = PRINT_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  Ok(a)
}

/// Parse a JUMP instruction.
fn parse_jump(line: &str) -> Result<String, Error> {
  let capt = JUMP_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  Ok(capt[1].to_owned())
}

/// Parse conditional jump (BGE, BGT, BLT, BLE, BGT, BGE) instruction.
fn parser_cond_jump(line: &str) -> Result<(usize, usize, String), Error> {
  let capt = COND_JUMP_PARSER
    .captures(line)
    .ok_or(Error::InvalidParameter)?;
  let a: usize = capt[1].parse().expect("error parsing");
  let b: usize = capt[2].parse().expect("error parsing");
  Ok((a, b, capt[3].to_owned()))
}

#[cfg(test)]
mod parse_test {
  use crate::{
    simulator::parser::{parse_instruction, process_lines},
    simulator::{Instructions, Simulator},
  };
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
    assert_eq!(x, Instructions::JUMP(String::from("ENDLOOP")));
  }

  #[test]
  fn parse_uncon_test() {
    let line: &str = "  BGE $4 $31 @ENDLOOP";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::BGE(4, 31, String::from("ENDLOOP")));
  }

  #[test]
  fn parse_print_test() {
    let line: &str = "  PRINT $4";
    let x = parse_instruction(line).unwrap();
    assert_eq!(x, Instructions::PRINT(4));
  }

  #[test]
  fn process_lines_test() {
    let mut simul = Simulator::new();
    let lines: Vec<String> = vec![
      String::from("LI $54 45"),
      String::from("PRINT $4"),
      String::from("BGE $1300 $23 @SOMETHING"),
    ];

    process_lines(&lines, &mut simul).expect("error found");
    assert_eq!(simul.instructions[0], Instructions::LI(54, 45));
    assert_eq!(simul.instructions[1], Instructions::PRINT(4));
    assert_eq!(
      simul.instructions[2],
      Instructions::BGE(1300, 23, String::from("SOMETHING"))
    );
  }
} // mod parse_test
