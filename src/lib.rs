extern crate proc_macro;

use proc_macro::token_stream::IntoIter;
use proc_macro::Span;


// A couple of approaches
// #Iterator → various kinds of Visitor
// #Direct Parse and Parser Combinator (and things like 'capturing', 'mapping' etc nodes, where a parser is a struct or enum with an `impl Ish`, and a combinator is a struct or enum with `impl Ish` that also has one or more members with `impl Ish`
// need a parser for Socket<impl Ishy>; it lifts out capturing etc from special cases
// TT etc in the direct mode are parsers that match on type and then visit into the socket
// Need to reconsider how Lit is factored
//
// Direct Parse is the one I'm focussing on first

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
impl<R, F, T> Ish<R, F> for Socket<T> where T: Ish<R, F> {
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}
pub enum TT {
  Item(Socket<Item>),
  Lit(Lit),
  Punct(Socket<Punct>),
  Group(Socket<Group>),
}
impl TT {
  fn item(it: &str) -> Self {
    TT::Item(Socket::Val(Item { name: it.to_string() }))
  }
  fn lit_f32(it: f32) -> Self {
    TT::Lit(Lit::Lf32(Socket::Val(it)))
  }
  fn lit_f64(it: f64) -> Self {
    TT::Lit(Lit::Lf64(Socket::Val(it)))
  }
  fn punct() -> Self {
    TT::Punct(Socket::DontCare)
  }
  fn group() -> Self {
    TT::Group(Socket::Val(Group { contents: vec!()}))
  }
}
impl<R, F> Ish<R, F> for TT {
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
    match self {
      TT::Item(socket) => {
        // check if it is a 
      },
      TT::Lit(socket) => {
        // check if it is a 
      },
      TT::Punct(socket) => {
        // check if it is a 
      },
      TT::Group(socket) => {
        // check if it is a 
      },
    }
    todo!()
  }
}
pub struct Item {
  pub name: String,
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
  Lunparsed(Socket<Box<str>>),
}
impl<R, F> Ish<R, F> for Lit {
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}
pub struct Punct {
  pub spacing: Socket<Spacing>,
  pub it: Socket<char>,
}
pub enum Spacing {
  Joint,
  Alone,
}
pub struct Group {
  pub contents: Vec<TT>,
}

pub struct Seq<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish<R, F> for Seq<R, F> {
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}

pub struct Alt<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish<R, F> for Alt<R, F> {
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
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
