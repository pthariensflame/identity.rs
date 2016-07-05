use std::marker::PhantomData;
use std::{fmt, hash};

pub trait TyFun<Param: ?Sized>: Sized {
  type Result: ?Sized;
}

impl<'x, Param: ?Sized, F: TyFun<Param>> TyFun<Param> for &'x F {
  type Result = F::Result;
}

impl<'x, Param: ?Sized, F: TyFun<Param>> TyFun<Param> for &'x mut F {
  type Result = F::Result;
}

pub trait LiFun<'param>: Sized {
  type Result: ?Sized;
}

impl<'x, 'param, F: LiFun<'param>> LiFun<'param> for &'x F {
  type Result = F::Result;
}

impl<'x, 'param, F: LiFun<'param>> LiFun<'param> for &'x mut F {
  type Result = F::Result;
}

pub struct LiToTy<'param> {
  phantom_rf: PhantomData<&'param ()>,
}

impl<'param> fmt::Debug for LiToTy<'param> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("LiToTy").field("phantom_rf", &self.phantom_rf).finish() }
}

impl<'param> Clone for LiToTy<'param> {
  fn clone(&self) -> Self { LiToTy { phantom_rf: self.phantom_rf.clone() } }

  fn clone_from(&mut self, source: &Self) { self.phantom_rf.clone_from(&source.phantom_rf); }
}

impl<'param> Copy for LiToTy<'param> {}

impl<'param> Default for LiToTy<'param> {
  fn default() -> Self { LiToTy { phantom_rf: PhantomData::default() } }
}

impl<'param> hash::Hash for LiToTy<'param> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.phantom_rf.hash(state);
  }
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct LiFunToTyFun<F> {
  inner: F,
}

impl<'param, F: LiFun<'param>> TyFun<LiToTy<'param>> for LiFunToTyFun<F> {
  type Result = F::Result;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct TyFunToLiFun<F> {
  inner: F,
}

impl<'param, F: TyFun<LiToTy<'param>>> LiFun<'param> for TyFunToLiFun<F> {
  type Result = F::Result;
}

/// Type-level function composition.
#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Compose<TFa, TFb> {
  tf_a: TFa,
  tf_b: TFb,
}

pub fn ty_compose<TFa, TFb>(tf_a: TFa, tf_b: TFb) -> Compose<TFa, TFb> { Compose { tf_a: tf_a, tf_b: tf_b } }

impl<Param: ?Sized, TFa, TFb> TyFun<Param> for Compose<TFa, TFb>
  where TFa: TyFun<Param>, TFb: TyFun<TFa::Result> {
  type Result = TFb::Result;
}

impl<'param, TFa, TFb> LiFun<'param> for Compose<TFa, TFb>
  where TFa: LiFun<'param>, TFb: TyFun<TFa::Result> {
  type Result = TFb::Result;
}

/// Type-level I combinator.
#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Id;

pub fn ty_id() -> Id { Id }

impl<Param: ?Sized> TyFun<Param> for Id {
  type Result = Param;
}

/// Type-level K combinator.
pub struct Const<X: ?Sized> {
  phantom_fn: PhantomData<fn(X) -> X>,
}

impl<X: ?Sized> fmt::Debug for Const<X> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("Const").field("phantom_fn", &self.phantom_fn).finish() }
}

impl<X: ?Sized> Clone for Const<X> {
  fn clone(&self) -> Self { Const { phantom_fn: self.phantom_fn.clone() } }

  fn clone_from(&mut self, source: &Self) { self.phantom_fn.clone_from(&source.phantom_fn); }
}

impl<X: ?Sized> Copy for Const<X> {}

impl<X: ?Sized> Default for Const<X> {
  fn default() -> Self { Const { phantom_fn: PhantomData::default() } }
}

impl<X: ?Sized> hash::Hash for Const<X> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.phantom_fn.hash(state);
  }
}

pub fn ty_const<X: ?Sized>() -> Const<X> { Const { phantom_fn: PhantomData } }

impl<Param: ?Sized, X: ?Sized> TyFun<Param> for Const<X> {
  type Result = X;
}

impl<'param, X: ?Sized> LiFun<'param> for Const<X> {
  type Result = X;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct ConstTyFun;

impl<X: ?Sized> TyFun<X> for ConstTyFun {
  type Result = Const<X>;
}

/// Type-level S combinator.
#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Ap<F, G> {
  inner_f: F,
  inner_g: G,
}

pub fn ty_ap<F, G>(f: F, g: G) -> Ap<F, G> { Ap { inner_f: f, inner_g: g } }

impl<Param: ?Sized, F, G> TyFun<Param> for Ap<F, G>
  where F: TyFun<Param>, G: TyFun<Param>, F::Result: TyFun<G::Result> {
  type Result = <F::Result as TyFun<G::Result>>::Result;
}

impl<'param, F, G> LiFun<'param> for Ap<F, G>
  where F: LiFun<'param>, G: LiFun<'param>, F::Result: TyFun<G::Result> {
  type Result = <F::Result as TyFun<G::Result>>::Result;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct ApTyFun0;

impl<F> TyFun<F> for ApTyFun0 {
  type Result = ApTyFun1<F>;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct ApTyFun1<F> {
  inner: F,
}

impl<F, G> TyFun<G> for ApTyFun1<F> {
  type Result = Ap<F, G>;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Flip<F> {
  inner: F,
}

impl<ParamB: ?Sized, F> TyFun<ParamB> for Flip<F> {
  type Result = Flipped<F, ParamB>;
}

pub struct Flipped<F, ParamB: ?Sized> {
  inner: F,
  phantom_fn: PhantomData<fn(ParamB) -> ParamB>,
}

impl<F: fmt::Debug, ParamB: ?Sized> fmt::Debug for Flipped<F, ParamB> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    fmtr.debug_struct("LiToTy").field("inner", &self.inner).field("phantom_fn", &self.phantom_fn).finish()
  }
}

impl<F: Clone, ParamB: ?Sized> Clone for Flipped<F, ParamB> {
  fn clone(&self) -> Self {
    Flipped {
      inner: self.inner.clone(),
      phantom_fn: self.phantom_fn.clone(),
    }
  }

  fn clone_from(&mut self, source: &Self) {
    self.inner.clone_from(&source.inner);
    self.phantom_fn.clone_from(&source.phantom_fn);
  }
}

impl<F: Copy, ParamB: ?Sized> Copy for Flipped<F, ParamB> {}

impl<F: Default, ParamB: ?Sized> Default for Flipped<F, ParamB> {
  fn default() -> Self {
    Flipped {
      inner: F::default(),
      phantom_fn: PhantomData::default(),
    }
  }
}

impl<F: hash::Hash, ParamB: ?Sized> hash::Hash for Flipped<F, ParamB> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.inner.hash(state);
    self.phantom_fn.hash(state);
  }
}

impl<ParamA: ?Sized, ParamB: ?Sized, F: TyFun<ParamA>> TyFun<ParamA> for Flipped<F, ParamB>
  where F::Result: TyFun<ParamB> {
  type Result = <F::Result as TyFun<ParamB>>::Result;
}

pub fn ty_flip<F>(f: F) -> Flip<F> { Flip { inner: f } }

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct FlipTyFun;

impl<F> TyFun<F> for FlipTyFun {
  type Result = Flip<F>;
}

pub trait ForAll<TF> {
  fn call<A>(self, x: A) -> TF::Result where TF: TyFun<A>, TF::Result: Sized;
}

pub trait Exists<TF>
  where TF: TyFun<Self::A>, TF::Result: Sized {
  type A;

  fn fst(self) -> Self::A;

  fn snd(self) -> TF::Result;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn flip_test() {
    struct TestFun0;
    impl TyFun<str> for TestFun0 {
      type Result = TestFun1;
    }
    struct TestFun1;
    impl TyFun<i32> for TestFun1 {
      type Result = bool;
    }
    let x: <<TestFun0 as TyFun<str>>::Result as TyFun<i32>>::Result = true;
    let y: <<Flip<TestFun0> as TyFun<i32>>::Result as TyFun<str>>::Result = false;
    assert!(x && !y)
  }
}
