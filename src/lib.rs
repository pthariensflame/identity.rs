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
  pub trait IdentityAux<A: ?Sized, B: ?Sized> {
    type InverseAux: ?Sized + super::Identity<B, A, InverseAux = Self>;

    fn conv_aux(&self, x: A) -> B where A: Sized, B: Sized;

    fn conv_ref_aux<'a>(&self, x: &'a A) -> &'a B;

    fn conv_mut_aux<'a>(&self, x: &'a mut A) -> &'a mut B;

    fn conv_box_aux(&self, x: Box<A>) -> Box<B>;

    fn conv_gen_aux<Tc: super::HasParam<A, B>>(&self, x: Tc) -> Tc::SubstParam where Tc::SubstParam: Sized;

    fn inv_aux(&self) -> &Self::InverseAux;
  }
}

/// An identity type; that is, the type of “equality witnesses.”
///
/// This trait is not actually extensible; it relies on a hidden auxiliary trait to operate
/// properly while maintaining its invariants.  It should instead be thought of as a semi-magical
/// bound that all concrete equality witness types satisfy.
pub trait Identity<A: ?Sized, B: ?Sized>: aux::IdentityAux<A, B> {
  fn conv(&self, x: A) -> B where A: Sized, B: Sized;

  fn conv_ref<'a>(&self, x: &'a A) -> &'a B;

  fn conv_mut<'a>(&self, x: &'a mut A) -> &'a mut B;

  fn conv_box(&self, x: Box<A>) -> Box<B>;

  fn conv_gen<Tc: HasParam<A, B>>(&self, x: Tc) -> Tc::SubstParam where Tc::SubstParam: Sized;

  fn inv(&self) -> &<Self as IdentityUtil<A, B>>::Inverse;
}

impl<A: ?Sized, B: ?Sized, Ev: ?Sized + aux::IdentityAux<A, B>> Identity<A, B> for Ev {
  fn conv(&self, x: A) -> B
    where A: Sized, B: Sized {
    self.conv_aux(x)
  }

  fn conv_ref<'a>(&self, x: &'a A) -> &'a B { self.conv_ref_aux(x) }

  fn conv_mut<'a>(&self, x: &'a mut A) -> &'a mut B { self.conv_mut_aux(x) }

  fn conv_box(&self, x: Box<A>) -> Box<B> { self.conv_box_aux(x) }

  fn conv_gen<Tc: HasParam<A, B>>(&self, x: Tc) -> Tc::SubstParam
    where Tc::SubstParam: Sized {
    self.conv_gen_aux(x)
  }

  fn inv(&self) -> &Ev::InverseAux { self.inv_aux() }
}

pub trait IdentityUtil<A: ?Sized, B: ?Sized>: Identity<A, B> {
  type Inverse: ?Sized + Identity<B, A>;
}

impl<A: ?Sized, B: ?Sized, Ev: ?Sized + Identity<A, B>> IdentityUtil<A, B> for Ev {
  type Inverse = Self::InverseAux;
}

pub struct Refl<A: ?Sized> {
  phantom_fn: PhantomData<fn(A) -> A>,
}

impl<A: ?Sized> Default for Refl<A> {
  fn default() -> Refl<A> { Refl { phantom_fn: PhantomData } }
}

impl<A: ?Sized> fmt::Debug for Refl<A> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("Refl").field("phantom_fn", &self.phantom_fn).finish() }
}

impl<A: ?Sized> aux::IdentityAux<A, A> for Refl<A> {
  type InverseAux = Self;

  fn conv_aux(&self, x: A) -> A
    where A: Sized {
    x
  }

  fn conv_ref_aux<'a>(&self, x: &'a A) -> &'a A { x }

  fn conv_mut_aux<'a>(&self, x: &'a mut A) -> &'a mut A { x }

  fn conv_box_aux(&self, x: Box<A>) -> Box<A> { x }

  fn inv_aux(&self) -> &Self { self }

  fn conv_gen_aux<Tc: HasParam<A, A>>(&self, x: Tc) -> Tc::SubstParam
    where Tc::SubstParam: Sized {
    x.conv_under(self)
  }
}

pub trait Equals<Other: ?Sized> {
  type IdentityWitness: Identity<Self, Other>;

  fn identity_witness() -> Self::IdentityWitness;
}

impl<T: ?Sized> Equals<T> for T {
  type IdentityWitness = Refl<T>;

  fn identity_witness() -> Refl<T> { Refl::default() }
}

pub trait HasParam<OldParam: ?Sized, NewParam: ?Sized> {
  type SubstParam: ?Sized + HasParam<NewParam, OldParam, SubstParam = Self>;

  fn conv_under<Ev: Identity<OldParam, NewParam>>(self, ev: &Ev) -> Self::SubstParam where Self: Sized, Self::SubstParam: Sized;
}

impl<'a, OldParam: ?Sized + 'a, NewParam: ?Sized + 'a> HasParam<OldParam, NewParam> for &'a OldParam {
  type SubstParam = &'a NewParam;

  fn conv_under<Ev: Identity<OldParam, NewParam>>(self, ev: &Ev) -> Self::SubstParam {
    ev.conv_ref(self)
  }
}

impl<'a, OldParam: ?Sized + 'a, NewParam: ?Sized + 'a> HasParam<OldParam, NewParam> for &'a mut OldParam {
  type SubstParam = &'a mut NewParam;

  fn conv_under<Ev: Identity<OldParam, NewParam>>(self, ev: &Ev) -> Self::SubstParam {
    ev.conv_mut(self)
  }
}

impl<OldParam: ?Sized, NewParam: ?Sized> HasParam<OldParam, NewParam> for Box<OldParam> {
  type SubstParam = Box<NewParam>;

  fn conv_under<Ev: Identity<OldParam, NewParam>>(self, ev: &Ev) -> Self::SubstParam {
    ev.conv_box(self)
  }
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
