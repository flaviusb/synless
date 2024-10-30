extern crate proc_macro;

use proc_macro;

// Result, Fail, CompileError, Accumulation
trait<R, F, C, A> Ish<R, F, C, A> {
  fn parse(self, it: IntoIter) -> ;
}
pub enum TT<...> {
  Item(),
  Lit(),
  Punct(),
  Group(),
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

