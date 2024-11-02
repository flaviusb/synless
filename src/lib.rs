extern crate proc_macro;

use proc_macro;

// Result, Fail, CompileError, Accumulation
// Accumulation is fn((R, Vec<Span>), (R, Vec<Span>)) -> (R, Vec<Span>)
pub enum Ishy<R, F> {
  Res(R, Vec<Span>),
  Fail(F, Span),
  CompileError(F, Span),
}
trait Ish<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F>;
}
type Key = &str;
pub enum Socket<Inner> {
  DontCare,
  Capture(Key),
  Val(Inner),
}
pub enum TT<R, F> {
  Item(Socket<Item<R, F>>),
  Lit(Socket<Lit<R, F>>),
  Punct(Socket<Punct<R, F>>),
  Group(Socket<Group<R, F>>),
}
trait TT<R, F> {
  fn item() -> Self {
    TT::<R, F>::Item(Socket::Val(Item {}))
  }
  fn lit() -> Self {
    TT::<R, F>::Lit(Socket::Val(Lit {}))
  }
  fn punct() -> Self {
    TT::<R, F>::Punct(Socket::DontCare)
  }
  fn group() -> Self {
    TT::<R, F>::Group(Socket::Val(Group {}))
  }
}
impl Ish<R, F> for TT<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}
pub struct Item {}
pub enum Lit {}
pub struct Punct {
  pub spacing: Socket<Spacing>,
  pub it: Socket<char>,
}
pub enum Spacing {
  Joint,
  Alone,
}
pub struct Group {}

pub struct Seq<R, F> {
  sequence: Vec<Box<Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl Ish<R, F> for Seq<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}

pub struct Alt<R, F> {
  sequence: Vec<Box<Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl Ish<R, F> for Alt<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}

/*
Seq
Alt
Not
Capture
Plus
Star
Maybe
TT
TTQ
Map
*/
