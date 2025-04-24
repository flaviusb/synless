extern crate proc_macro;

use proc_macro::token_stream::{IntoIter};
use proc_macro::{Span, Group, Spacing, Delimiter, Punct, TokenStream, TokenTree};

pub trait Pattern<Accumulator: Clone> {
  fn run(&self, a: Accumulator, ts: TokenStream) -> (bool, TokenStream, Accumulator);
}

pub struct MapAcc<A: Clone, B: Clone> {
  pub it: fn(A) -> B,
  pub inner_acc: A,
  pub inner: dyn Pattern<A>,
}

impl<A: Clone, B: Clone> Pattern<B> for MapAcc<A, B> {
  fn run(&self, b: B, ts: TokenStream) -> (bool, TokenStream, B) {
    let (m, t, a) = self.inner.run(self.inner_acc.clone(), ts);
    return (m, t, (self.it)(a));
  }
}

pub fn transform<Accumulator: Clone, T: Pattern<Accumulator>>(pat: &T, accumulator: Accumulator, map_it: fn(Accumulator) -> TokenStream, input_ts: TokenStream) -> TokenStream {
  let mut result_ts = TokenStream::new();
  let mut ts = input_ts.into_iter();
  loop {
    match ts.next() {
      Some(TokenTree::Group(grp)) => {
        result_ts.extend(TokenStream::from(TokenTree::Group(Group::new(grp.delimiter(), transform(pat, accumulator.clone(), map_it, grp.stream())))).into_iter());
      },
      Some(it) => {
        let it_ts = TokenStream::from(it.clone());
        if let (true, ts_new, acc_new) = pat.run(accumulator.clone(), it_ts.clone()) {
          result_ts.extend(map_it(acc_new));
          ts = ts_new.into_iter();
        } else {
          result_ts.extend(it_ts.into_iter());
        }
      },
      None => break,
    };
  }
  return result_ts;
}

#[derive(Clone)]
pub enum S<A: Clone, T: Clone> {
  DontCare,
  Get(fn(A, T) -> A),
  Is(T),
  Match(fn(T) -> bool),
  MatchIs(T, fn(T) -> bool),
  MatchGet(fn(T) -> bool, fn(A, T) -> A),
}


macro_rules! s_inner {
  ($matchon:expr, $val:expr, $ts:expr, $acc:expr, $new_acc:expr) => {
      match $matchon {
        S::DontCare => {
          // Don't need to do anything here
        },
        S::Is(x) => {
          if $val != x {
            return (false, $ts, $acc);
          }
        },
        S::Get(getter) => {
          $new_acc = getter($new_acc, $val);
        },
        S::Match(check) => {
          if !check($val) {
            return (false, $ts, $acc);
          }
        },
        S::MatchIs(x, check) => {
          if ($val != x) || (!check($val))  {
            return (false, $ts, $acc);
          }
        },
        S::MatchGet(check, getter) => {
          if check($val) {
            $new_acc = getter($new_acc, $val);
          } else {
            return (false, $ts, $acc);
          }
        },
      };
  }
}



pub struct SPunct<A: Clone> {
  pub which: S<A, char>,
  pub spacing: S<A, Spacing>,
  pub inner_acc: A,
}

pub struct SGroup<A: Clone> {
  pub delimiter: S<A, Delimiter>,
  pub stream: S<A, TokenStream>,
  pub inner_acc: A,
}

pub struct SIdent<A: Clone> {
  pub string: S<A, String>,
  pub inner_acc: A,
}

pub enum SLiteral<A: Clone> {
  //Uninterp(A, S<A, Literal>),
  U8(A,    S<A, u8>),
  U16(A,   S<A, u16>),
  U32(A,   S<A, u32>),
  U64(A,   S<A, u64>),
  U128(A,  S<A, u128>),
  I8(A,    S<A, i8>),
  I16(A,   S<A, i16>),
  I32(A,   S<A, i32>),
  I64(A,   S<A, i64>),
  I128(A,  S<A, i128>),
  F32(A,   S<A, f32>),
  F64(A,   S<A, f64>),
  Usize(A, S<A, usize>),
  Isize(A, S<A, isize>),
  Char(A,  S<A, char>),
  RString(A, S<A, String>),
  //BString(A, S<A, &[u8]>),
  CString(A, S<A, std::ffi::CString>),
}

impl<A: Clone> Pattern<A> for SPunct<A> {
  fn run(&self, acc: A, ts: TokenStream) -> (bool, TokenStream, A) {
    let mut ts_iter = ts.clone().into_iter();
    match ts_iter.next() {
      Some(TokenTree::Punct(punct)) => {
        let mut new_acc = acc.clone();
        s_inner!(self.which,   punct.as_char(), ts, acc, new_acc);
        s_inner!(self.spacing, punct.spacing(), ts, acc, new_acc);
        let mut ts_out = TokenStream::new();
        ts_out.extend(ts_iter);
        return (true, ts_out, new_acc);
      },
      Some(x) => {
        return (false, ts, acc);
      },
      None => {
        return (false, TokenStream::new(), acc);
      },
    }
  }
}

impl<A: Clone> Pattern<A> for SIdent<A> {
  fn run(&self, acc: A, ts: TokenStream) -> (bool, TokenStream, A) {
    let mut ts_iter = ts.clone().into_iter();
    match ts_iter.next() {
      Some(TokenTree::Ident(ident)) => {
        let mut new_acc = acc.clone();
        s_inner!(self.string.clone(), ident.to_string(), ts, acc, new_acc);
        let mut ts_out = TokenStream::new();
        ts_out.extend(ts_iter);
        return (true, ts_out, new_acc);
      },
      Some(x) => {
        return (false, ts, acc);
      },
      None => {
        return (false, TokenStream::new(), acc);
      },
    }
  }
}

//#[test]
pub fn test_punct_match() {
  let dollar_alone = SPunct::<Option<bool>> {
    which: S::Is('$'),
    spacing: S::Is(Spacing::Alone),
    inner_acc: None,
  };
  let dollar_meh = SPunct::<Option<bool>> {
    which: S::Is('$'),
    spacing: S::DontCare,
    inner_acc: None,
  };
  #[derive(Clone)]
  struct dollar_acc {
    joined: bool,
    amount: u8,
  }
  fn dollar_getter(acc: dollar_acc, sp: Spacing) -> dollar_acc {
    match sp {
      Spacing::Joint => {
        dollar_acc { joined: true,  amount: acc.amount + 1, }
      },
      Spacing::Alone => {
        dollar_acc { joined: false, amount: acc.amount, }
      }
    }
  }
  let dollar_get = SPunct::<dollar_acc> {
    which: S::Is('$'),
    spacing: S::Get(dollar_getter),
    inner_acc: dollar_acc { joined: false, amount: 0, },
  };
  let mut tt = TokenStream::new();
  tt.extend(TokenStream::from(TokenTree::Punct(Punct::new('$', Spacing::Alone))));
  let (m1, tr1, res1) = dollar_alone.run(None, tt.clone());
  let (m2, tr2, res2) = dollar_meh.run(None, tt.clone());
  let (m3, tr3, res3) = dollar_get.run(dollar_acc { joined: false, amount: 0 }, tt.clone());
}

/*

trait Parse
trait Answer


let foo = Seq { seq: vec!( Alt { options: vec!{ Lit::unparsed("0"), Lit::unparsed("1"),  } } ) };
let foo = seq([alt([u("0"), u("1")])])
Star
Plus
Questionmark


let out = foo.parse(the_input);

fn's as members for Map {}, Filter {}, ...

*/

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

/*
pub enum Ishy<R, F> {
  Res(R, Vec<Span>),
  Fail(F, Option<Span>),
  CompileError(F, Option<Span>),
}
trait Ish {
  type ResultType;
  type FailType;
  fn parse(self, it: IntoIter) -> Ishy<self::ResultType, self::FailType>;
}
type Key = str;
pub enum Socket<Inner> {
  DontCare,
  Capture(Box<Key>),
  Val(Inner),
}
pub enum SocketFailure {
  Eh,
  EOS,
  NoMatch,
}
pub enum SocketSuccess<T> {
  Eh,
  KV(Box<Key>, T),
}

impl<T: Ish> Ish<SocketSuccess<T>, SocketFailure> for Socket<T> {
  fn parse(self, mut it: IntoIter) -> Ishy<SocketSuccess<T>, SocketFailure> {
    if let Some(res) = it.next() {
      match self {
        Socket::DontCare     => return Ishy::Res(SocketSuccess::Eh, vec!(res.span())),
        Socket::Val(inner)   => todo!(), //if inner == res { return Ishy::Res(SocketSuccess::Eh, res.span()) } else { return Ishy::Fail(SocketFailure::NoMatch, it.span()) },
        Socket::Capture(key) => todo!(),
      }
    } else {
      return Ishy::Fail(SocketFailure::EOS, None);
    }
  }
}
pub enum TT {
  Item(Socket<Item>),
  Lit(Socket<Lit>),
  Punct(Socket<Punct>),
  Group(Socket<Group>),
}
impl TT {
  fn item(it: &str) -> Self {
    TT::Item(Socket::Val(Item { name: it.to_string() }))
  }
  fn lit_f32(it: f32) -> Self {
    TT::Lit(Socket::Val(Lit::Lf32(Socket::Val(it))))
  }
  fn lit_f64(it: f64) -> Self {
    TT::Lit(Socket::Val(Lit::Lf64(Socket::Val(it))))
  }
  fn punct() -> Self {
    TT::Punct(Socket::DontCare)
  }
  fn group() -> Self {
    TT::Group(Socket::Val(Group { contents: Socket::Val(vec!()), delimeter: Socket::Val(Delimeter::Parenthesis) }))
  }
}
impl<R, F> Ish for TT {
  type ResultType = R;
  type FailType = F;
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
impl<R, F> Ish for Lit {
  type ResultType = R;
  type FailType = F;
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
  pub contents: Socket<Vec<TT>>,
  pub delimeter: Socket<Delimeter>,
}
pub enum Delimeter {
  Parenthesis,
  Brace,
  Bracket,
  None,
}

pub struct Seq<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish for Seq<R, F> {
  type ResultType = R;
  type FailType = F;
  fn parse(self, mut it: IntoIter) -> Ishy<R, F> {
    todo!()
  }
}

pub struct Alt<R, F> {
  sequence: Vec<Box<dyn Ish<R, F>>>,
  no_match: Option<fn((R, Vec<Span>)) -> (F, Vec<Span>)>,
}
impl<R, F> Ish for Alt<R, F> {
  type ResultType = R;
  type FailType = F;
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


/*

  IntoIter next

*/

*/
