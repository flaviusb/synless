extern crate proc_macro;

use proc_macro;

// Result, Fail, CompileError, Accumulation
// Accumulation is fn(R, Vec<Span>, R, Vec<Span>) -> (R, Vec<Span>)
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
pub enum TT<...> {
  Item(Socket<Item>),
  Lit(Socket<Lit>),
  Punct(Socket<Punct>),
  Group(Socket<Group>),
}
trait<..> TT<..> {
  fn item() ->
  fn lit() ->
  fn punct() ->
  fn group() ->
}
trait<...> Ish<...> for TT<...> {
  fn parse(self, it: IntoIter) -> ;
}
pub struct Item;
pub enum Lit;
pub struct Punct {
  pub spacing: Socket<Spacing>,
  pub it: Socket<char>,
}
pub enum Spacing {
  Joint,
  Alone,
}
pub struct Group;

pub struct Seq<T, ...> {
  sequence: Vec<T>,
  no_match: Option<fn ...>,
}
trait Ish for Seq... {
  fn parse(self, it: IntoIter) -> ;
}

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

