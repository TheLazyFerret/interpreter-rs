//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! operations related module

use crate::simulator::Simulator;
use crate::{Error, Instructions};
use std::collections::HashMap;
use std::process;

/// ADD operation. params.0 = params.1 + params.2
pub fn addition(sim: &mut Simulator, params: (usize, usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 || params.2 >= 32 {
    Err(Error::OutOfRange)
  } else {
    let result: i32 = sim
      .get_int_reg(params.1)
      .wrapping_add(sim.get_int_reg(params.2));
    sim.set_int_reg(params.0, result);
    Ok(())
  }
}

/// SUB operation. params.0 = params.1 - params.2
pub fn substraction(sim: &mut Simulator, params: (usize, usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 || params.2 >= 32 {
    Err(Error::OutOfRange)
  } else {
    let result: i32 = sim
      .get_int_reg(params.1)
      .wrapping_sub(sim.get_int_reg(params.2));
    sim.set_int_reg(params.0, result);
    Ok(())
  }
}

/// MUL operation. params.0 = params.1 * params.2
pub fn multiplication(sim: &mut Simulator, params: (usize, usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 || params.2 >= 32 {
    Err(Error::OutOfRange)
  } else {
    let result: i32 = sim
      .get_int_reg(params.1)
      .wrapping_mul(sim.get_int_reg(params.2));
    sim.set_int_reg(params.0, result);
    Ok(())
  }
}

/// DIV operation. params.0 = params.1 / params.2
pub fn division(sim: &mut Simulator, params: (usize, usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 || params.2 >= 32 {
    Err(Error::OutOfRange)
  } else if params.2 == 0 {
    Err(Error::DivisionByZero)
  } else {
    let result: i32 = sim
      .get_int_reg(params.1)
      .wrapping_div(sim.get_int_reg(params.2));
    sim.set_int_reg(params.0, result);
    Ok(())
  }
}

/// REM operation. params.0 = params.1 % params.2
pub fn remain(sim: &mut Simulator, params: (usize, usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 || params.2 >= 32 {
    Err(Error::OutOfRange)
  } else if params.2 == 0 {
    Err(Error::DivisionByZero)
  } else {
    let result: i32 = sim
      .get_int_reg(params.1)
      .wrapping_rem(sim.get_int_reg(params.2));
    sim.set_int_reg(params.0, result);
    Ok(())
  }
}

/// LI operation. params.0 = params.1 (imm)
pub fn load_integer(sim: &mut Simulator, params: (usize, i32)) -> Result<(), Error> {
  if params.0 >= 32 {
    Err(Error::OutOfRange)
  } else {
    sim.set_int_reg(params.0, params.1);
    Ok(())
  }
}

/// MOVE (copy) operation. params.0 = params.1
pub fn move_reg(sim: &mut Simulator, params: (usize, usize)) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 {
    Err(Error::OutOfRange)
  } else {
    sim.set_int_reg(params.0, sim.get_int_reg(params.1));
    Ok(())
  }
}

/// PRINT operation. print params
pub fn print_reg(sim: &Simulator, params: usize) -> Result<(), Error> {
  if params >= 32 {
    Err(Error::OutOfRange)
  } else {
    println!("PRINT => ${}: {}", params, sim.get_int_reg(params));
    Ok(())
  }
}

/// JUMP operation. Set ic to labels's value
pub fn unc_jump(
  sim: &mut Simulator, labels: &HashMap<String, usize>, params: &str,
) -> Result<(), Error> {
  let value = labels.get(params);

  if let Some(lab) = value {
    sim.set_ic(lab.to_owned());
    Ok(())
  } else {
    Err(Error::UnknownLabel)
  }
}

/// Conditional jump operations (BEQ, BNE, BLT, BLE, BGT, BGE). Set's ic to label's value if the condition is true
pub fn con_jump(
  sim: &mut Simulator, lab: &HashMap<String, usize>, params: (usize, usize, String),
  instr: Instructions,
) -> Result<(), Error> {
  if params.0 >= 32 || params.1 >= 32 {
    Err(Error::OutOfRange)
  } else if lab.get(&params.2).is_none() {
    Err(Error::UnknownLabel)
  } else {
    let a = sim.get_int_reg(params.0);
    let b = sim.get_int_reg(params.1);
    let jump = lab.get(&params.2).unwrap().to_owned();

    match instr {
      Instructions::BEQ => {
        if a == b {
          sim.set_ic(jump)
        }
      }
      Instructions::BNE => {
        if a != b {
          sim.set_ic(jump)
        }
      }
      Instructions::BLT => {
        if a < b {
          sim.set_ic(jump)
        }
      }
      Instructions::BLE => {
        if a <= b {
          sim.set_ic(jump)
        }
      }
      Instructions::BGT => {
        if a > b {
          sim.set_ic(jump)
        }
      }
      Instructions::BGE => {
        if a >= b {
          sim.set_ic(jump)
        }
      }
      _ => return Err(Error::InvalidParameter),
    };
    Ok(())
  }
} // fn con_jump

/// EXIT operation.
pub fn exit() {
  println!("EXIT");
  process::exit(0);
}
