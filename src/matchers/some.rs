// Copyright 2018 Val Markovic
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;

use core::*;

pub struct IsSome<T> {
  marker: PhantomData<T>,
}

impl<T> fmt::Display for IsSome<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Some(_)")
  }
}

impl<T: fmt::Debug> Matcher<Option<T>> for IsSome<T> {
  fn matches(&self, actual: Option<T>) -> MatchResult {
    match actual {
      None => Err(format!("was None")),
      Some(_) => success(),
    }
  }
}

pub fn some<T>() -> IsSome<T> {
  IsSome {
    marker: PhantomData,
  }
}
