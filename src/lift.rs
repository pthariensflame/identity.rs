pub trait TyFun<Param: ?Sized>: Sized {
  type Result: ?Sized;
}

pub trait LiFun<'param>: Sized {
  type Result: ?Sized;
}

#[derive(Debug,Clone,Copy,Default)]
pub struct Compose<TFa, TFb> {
  tf_a: TFa,
  tf_b: TFb,
}

pub fn compose<TFa, TFb>(tf_a: TFa, tf_b: TFb) -> Compose<TFa, TFb> { Compose { tf_a: tf_a, tf_b: tf_b } }

impl<Param: ?Sized, TFa, TFb> TyFun<Param> for Compose<TFa, TFb>
  where TFa: TyFun<Param>, TFb: TyFun<TFa::Result> {
  type Result = TFb::Result;
}

impl<'param, TFa, TFb> LiFun<'param> for Compose<TFa, TFb>
  where TFa: LiFun<'param>, TFb: TyFun<TFa::Result> {
  type Result = TFb::Result;
}

#[derive(Debug,Clone,Copy,Default)]
pub struct Id;

pub fn id() -> Id { Id }

impl<Param: ?Sized> TyFun<Param> for Id {
  type Result = Param;
}
