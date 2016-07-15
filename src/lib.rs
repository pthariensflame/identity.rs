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
use std::{fmt, hash};

#[macro_use]
pub mod lift;

/// An identity type; that is, the type bound of “equality witnesses.”
pub trait Identity<A: ?Sized, B: ?Sized>: Sized {
  type Inverse: Identity<B, A, Inverse = Self>;

  fn conv(&self, x: A) -> B where A: Sized, B: Sized;

  fn conv_ref<'a>(&self, x: &'a A) -> &'a B;

  fn conv_mut<'a>(&self, x: &'a mut A) -> &'a mut B;

  fn conv_box(&self, x: Box<A>) -> Box<B>;

  /// Leibniz's identity rule, approximately.
  fn conv_under<TF: lift::TyFun<A> + lift::TyFun<B>>(&self, x: <TF as lift::TyFun<A>>::Result) -> <TF as lift::TyFun<B>>::Result
    where <TF as lift::TyFun<A>>::Result: Sized, <TF as lift::TyFun<B>>::Result: Sized;

  fn inv(&self) -> &Self::Inverse;

  /// Paulin-Mohring's J rule, approximately.
  fn elim<Prop: lift::TyFun2<A, Refl<A>> + lift::TyFun2<B, Self>>
    (&self, refl_case: <Prop as lift::TyFun2<A, Refl<A>>>::Result) -> <Prop as lift::TyFun2<B, Self>>::Result
    where <Prop as lift::TyFun2<A, Refl<A>>>::Result: Sized, <Prop as lift::TyFun2<B, Self>>::Result: Sized;
}

pub fn refl<A: ?Sized>() -> Refl<A> { Refl::default() }

pub struct Refl<A: ?Sized> {
  phantom_fn: PhantomData<fn(A) -> A>,
}

impl<A: ?Sized> fmt::Debug for Refl<A> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("Refl").field("phantom_fn", &self.phantom_fn).finish() }
}

impl<A: ?Sized> Clone for Refl<A> {
  fn clone(&self) -> Refl<A> { refl() }
}

impl<A: ?Sized> Copy for Refl<A> {}

impl<A: ?Sized> Default for Refl<A> {
  fn default() -> Refl<A> { Refl { phantom_fn: PhantomData } }
}

impl<A: ?Sized> hash::Hash for Refl<A> {
  fn hash<H: hash::Hasher>(&self, hshr: &mut H) { self.phantom_fn.hash(hshr); }
}

impl<A: ?Sized> Identity<A, A> for Refl<A> {
  type Inverse = Self;

  fn conv(&self, x: A) -> A
    where A: Sized {
    x
  }

  fn conv_ref<'a>(&self, x: &'a A) -> &'a A { x }

  fn conv_mut<'a>(&self, x: &'a mut A) -> &'a mut A { x }

  fn conv_box(&self, x: Box<A>) -> Box<A> { x }

  fn inv(&self) -> &Self { self }

  fn conv_under<TF: lift::TyFun<A>>(&self, x: TF::Result) -> TF::Result
    where TF::Result: Sized {
    x
  }

  fn elim<Prop: lift::TyFun2<A, Refl<A>>>(&self,
                                          refl_case: <Prop as lift::TyFun2<A, Refl<A>>>::Result)
                                          -> <Prop as lift::TyFun2<A, Refl<A>>>::Result
    where <Prop as lift::TyFun2<A, Refl<A>>>::Result: Sized {
    refl_case
  }
}

pub trait Equals<Other: ?Sized> {
  type IdentityWitness: Identity<Self, Other>;

  fn identity_witness() -> Self::IdentityWitness;
}

impl<T: ?Sized> Equals<T> for T {
  type IdentityWitness = Refl<T>;

  fn identity_witness() -> Refl<T> { refl() }
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
