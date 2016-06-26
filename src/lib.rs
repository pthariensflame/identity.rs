// lib.rs
// Copyright 2016 Alexander Altman
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;
use std::fmt;

mod aux {
  pub trait IdentityAux<A, B> {
    type Inverse: super::Identity<B, A>;

    fn conv_aux(&self, x: A) -> B;

    fn inv_aux(&self) -> &Self::Inverse;
  }
}

pub trait Identity<A, B>: aux::IdentityAux<A, B> {
  fn conv(&self, x: A) -> B;

  fn inv(&self) -> &Self::Inverse;
}

impl<A, B, Ev: aux::IdentityAux<A, B>> Identity<A, B> for Ev {
  fn conv(&self, x: A) -> B { self.conv_aux(x) }

  fn inv(&self) -> &Ev::Inverse { self.inv_aux() }
}

pub struct Refl<A> {
  phantom_fn: PhantomData<fn(A) -> A>,
}

impl<A> Default for Refl<A> {
  fn default() -> Refl<A> { Refl { phantom_fn: PhantomData } }
}

impl<A> fmt::Debug for Refl<A> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("Refl").field("phantom_fn", &self.phantom_fn).finish() }
}

impl<A> aux::IdentityAux<A, A> for Refl<A> {
  type Inverse = Self;

  fn conv_aux(&self, x: A) -> A { x }

  fn inv_aux(&self) -> &Self { self }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn inv_conv_test() {
    fn inv_conv<A, B, Ev: Identity<A, B>>(ev: Ev, x: B) -> A { ev.inv().conv(x) }

    assert_eq!(inv_conv(Refl::default(), 0), 0)
  }
}
