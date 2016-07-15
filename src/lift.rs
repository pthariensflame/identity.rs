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

impl<Param: ?Sized, F: TyFun<Param>> TyFun<Param> for Box<F> {
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

impl<'param, F: LiFun<'param>> LiFun<'param> for Box<F> {
  type Result = F::Result;
}

pub struct LiToTy<'param> {
  phantom_fn: PhantomData<fn(&'param ()) -> &'param ()>,
}

impl<'param> fmt::Debug for LiToTy<'param> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result { fmtr.debug_struct("LiToTy").field("phantom_fn", &self.phantom_fn).finish() }
}

impl<'param> Clone for LiToTy<'param> {
  fn clone(&self) -> Self { LiToTy::default() }
}

impl<'param> Copy for LiToTy<'param> {}

impl<'param> Default for LiToTy<'param> {
  fn default() -> Self { LiToTy { phantom_fn: PhantomData } }
}

impl<'param> hash::Hash for LiToTy<'param> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.phantom_fn.hash(state);
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

/// Type-level function composition (traditional order).
#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Compose<TFa, TFb> {
  tf_a: TFa,
  tf_b: TFb,
}

/// Type-level function composition (intuitive order).
pub type AndThen<TFa, TFb> = Compose<TFb, TFa>;

pub fn ty_compose<TFa, TFb>(tf_a: TFa, tf_b: TFb) -> Compose<TFa, TFb> { Compose { tf_a: tf_a, tf_b: tf_b } }

pub fn ty_and_then<TFa, TFb>(tf_a: TFa, tf_b: TFb) -> AndThen<TFa, TFb> { Compose { tf_a: tf_b, tf_b: tf_a } }

impl<Param: ?Sized, TFa, TFb> TyFun<Param> for Compose<TFa, TFb>
  where TFb: TyFun<Param>, TFa: TyFun<TFb::Result> {
  type Result = TFa::Result;
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
  fn clone(&self) -> Self { Const::default() }
}

impl<X: ?Sized> Copy for Const<X> {}

impl<X: ?Sized> Default for Const<X> {
  fn default() -> Self { Const { phantom_fn: PhantomData } }
}

impl<X: ?Sized> hash::Hash for Const<X> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.phantom_fn.hash(state);
  }
}

pub fn ty_const<X: ?Sized>() -> Const<X> { Const::default() }

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
    fmtr.debug_struct("Flipped").field("inner", &self.inner).field("phantom_fn", &self.phantom_fn).finish()
  }
}

impl<F: Clone, ParamB: ?Sized> Clone for Flipped<F, ParamB> {
  fn clone(&self) -> Self {
    Flipped {
      inner: self.inner.clone(),
      phantom_fn: PhantomData,
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
      phantom_fn: PhantomData,
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

pub trait Pi<TF> {
  fn call<Param>(self, x: Param) -> TF::Result where TF: TyFun<Param>, TF::Result: Sized;
}

pub trait Sigma<TF>
  where TF: TyFun<Self::Param>, TF::Result: Sized {
  type Param;

  fn fst(self) -> Self::Param;

  fn snd(self) -> TF::Result;
}

pub trait Forall<TF> {
  fn instance<Param: ?Sized>(self) -> TF::Result where TF: TyFun<Param>, TF::Result: Sized;
}

pub trait Exists<TF>
  where TF: TyFun<Self::Param>, TF::Result: Sized {
  type Param: ?Sized;

  fn value(self) -> TF::Result;
}

impl<TF, X: Forall<TF>> Pi<TF> for X {
  fn call<Param>(self, _: Param) -> TF::Result
    where TF: TyFun<Param>, TF::Result: Sized {
    self.instance()
  }
}

impl<TF, X: Sigma<TF>> Exists<TF> for X
  where TF: TyFun<X::Param>, TF::Result: Sized {
  type Param = X::Param;

  fn value(self) -> TF::Result { self.snd() }
}

mod aux {
  pub trait TyListAux: Sized + ::std::fmt::Debug + Clone + Copy + Default + ::std::hash::Hash {}
}

pub trait TyList: aux::TyListAux {}

impl<L: aux::TyListAux> TyList for L {}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Nil;

impl aux::TyListAux for Nil {}

pub struct Cons<A: ?Sized, As: TyList> {
  phantom_fn: PhantomData<fn(A) -> A>,
  rest: As,
}

impl<A: ?Sized, As: TyList> fmt::Debug for Cons<A, As> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    fmtr.debug_struct("Cons").field("phantom_fn", &self.phantom_fn).field("rest", &self.rest).finish()
  }
}

impl<A: ?Sized, As: TyList> Clone for Cons<A, As> {
  fn clone(&self) -> Self {
    Cons {
      phantom_fn: PhantomData,
      rest: self.rest.clone(),
    }
  }

  fn clone_from(&mut self, source: &Self) {
    self.phantom_fn.clone_from(&source.phantom_fn);
    self.rest.clone_from(&source.rest);
  }
}

impl<A: ?Sized, As: TyList> Copy for Cons<A, As> {}

impl<A: ?Sized, As: TyList> Default for Cons<A, As> {
  fn default() -> Self {
    Cons {
      phantom_fn: PhantomData,
      rest: As::default(),
    }
  }
}

impl<A: ?Sized, As: TyList> hash::Hash for Cons<A, As> {
  fn hash<H>(&self, state: &mut H)
    where H: hash::Hasher {
    self.phantom_fn.hash(state);
    self.rest.hash(state);
  }
}

impl<A: ?Sized, As: TyList> aux::TyListAux for Cons<A, As> {}

// waiting on rust-lang/rust#34303 to allow lifetimes as well as types
#[macro_export]
macro_rules! ty_list {
  () => { $crate::lift::Nil };
  ($A:ty, $As:tt) => { $crate::lift::Cons<$A, ty_list!($As)> };
}

pub trait TysFun<Params: TyList> {
  type Result: ?Sized;
}

impl<R: ?Sized> TysFun<Nil> for R {
  type Result = R;
}

impl<Param: ?Sized, Params: TyList, F: TyFun<Param>> TysFun<Cons<Param, Params>> for F
  where F::Result: TysFun<Params> {
  type Result = <F::Result as TysFun<Params>>::Result;
}

#[derive(Debug,Clone,Copy,Default,Hash)]
pub struct Uncurry<F> {
  inner: F,
}

impl<Params: TyList, F: TysFun<Params>> TyFun<Params> for Uncurry<F> {
  type Result = F::Result;
}

pub type TyPair<A: ?Sized, B: ?Sized> = Cons<A, Cons<B, Nil>>;

pub type TyTriple<A: ?Sized, B: ?Sized, C: ?Sized> = Cons<A, Cons<B, Cons<C, Nil>>>;

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
    let x: <TestFun0 as TysFun<TyPair<str, i32>>>::Result = true;
    let y: <Flip<TestFun0> as TysFun<TyPair<i32, str>>>::Result = false;
    assert!(x && !y)
  }
}
