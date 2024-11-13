extern crate proc_macro;

use proc_macro::token_stream::IntoIter;
use proc_macro::Span;

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
type Key = str;
pub enum Socket<Inner> {
  DontCare,
  Capture(Box<Key>),
  Val(Inner),
}
pub enum TT<R, F> {
  Item(Socket<Item>),
  Lit(Socket<Lit>),
  Punct(Socket<Punct>),
  Group(Socket<Group<R, F>>),
}
impl<R, F> TT<R, F> {
  fn item(it: &str) -> Self {
    TT::<R, F>::Item(Socket::Val(Item { name: Box::new(*it) }))
  }
  fn lit_f32(it: f32) -> Self {
    TT::<R, F>::Lit(Socket::Val(Lit::Lf32(Socket::Val(it))))
  }
  fn lit_f64(it: f64) -> Self {
    TT::<R, F>::Lit(Socket::Val(Lit::Lf64(Socket::Val(it))))
  }
  fn punct() -> Self {
    TT::<R, F>::Punct(Socket::DontCare)
  }
  fn group() -> Self {
    TT::<R, F>::Group(Socket::Val(Group::<R, F> { contents: vec!()}))
  }
}
impl<R, F> Ish<R, F> for TT<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}
pub struct Item {
  pub name: Box<str>,
}
pub enum Lit {
  Lf32(Socket<f32>),
  Lf64(Socket<f64>),
  Lu8(Socket<u8>),
  Lu16(Socket<u16>),
  Lu32(Socket<u32>),
  Lu64(Socket<u64>),
  Lu128(Socket<u128>),
  Li8(Socket<i8>),
  Li16(Socket<i16>),
  Li32(Socket<i32>),
  Li64(Socket<i64>),
  Li128(Socket<i128>),
  Lchar(Socket<char>),
  Lstr(Socket<Box<str>>),
}
pub struct Punct {
  pub spacing: Socket<Spacing>,
  pub it: Socket<char>,
}
pub enum Spacing {
  Joint,
  Alone,
}
pub struct Group<R, F> {
  pub contents: Vec<TT<R, F>>,
}

pub struct Seq<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish<R, F> for Seq<R, F> {
  fn parse(self, it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}

pub struct Alt<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish<R, F> for Alt<R, F> {
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
