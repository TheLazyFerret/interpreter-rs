//! Author: TheLazyFerret (https://github.com/TheLazyFerret)
//! Copyright (c) 2025 TheLazyFerret
//! Licensed under the MIT License. See LICENSE file in the project root for full license information.
//!
//! stack related module

use std::collections::LinkedList;

#[derive(Clone, Debug, Default)]
pub struct Stack<T> {
  list: LinkedList<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Stack {
      list: LinkedList::new(),
    }
  }

  pub fn push(&mut self, value: T) {
    self.list.push_front(value);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.list.pop_front()
  }
}
