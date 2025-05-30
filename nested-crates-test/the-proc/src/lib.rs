extern crate synless;
extern crate proc_macro;

use synless::*;
use proc_macro::{TokenStream, TokenTree, Punct, Literal, Spacing, Delimiter, Group, Span, Ident};
use std::rc::Rc;


#[proc_macro]
pub fn dollar_increment(t: TokenStream) -> TokenStream {
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
  let mut tt = t.clone(); //TokenStream::new();
  //tt.extend(TokenStream::from(TokenTree::Punct(Punct::new('$', Spacing::Alone))));
  let (m1, tr1, res1) = dollar_alone.run(None, tt.clone());
  let (m2, tr2, res2) = dollar_meh.run(None, tt.clone());
  let (m3, tr3, res3) = dollar_get.run(dollar_acc { joined: false, amount: 0 }, tt.clone());
  #[derive(Clone)]
  struct dollar_num_acc {
    var: u32,
  }
  let dollar_meh2 = SPunct {
    which: S::Is('$'),
    spacing: S::DontCare,
    inner_acc: dollar_num_acc { var: 0 },
  };
  fn number_getter(acc: dollar_num_acc, num: u32) -> dollar_num_acc {
    dollar_num_acc { var: num }
  }
  let num_it = SLiteral::U32(dollar_num_acc { var: 0 }, S::Get(number_getter));
  let dollar_num = Seq { seq: Rc::new(vec!(Box::new(dollar_meh2), Box::new(num_it))), inner_acc: dollar_num_acc { var: 0 }, };
  let mut tt2 = TokenStream::new();
  tt2.extend(TokenStream::from(TokenTree::Punct(Punct::new('$', Spacing::Alone))));
  tt2.extend(TokenStream::from(TokenTree::Literal(Literal::u32_unsuffixed(7))));
  let (m4, tr4, res4) = dollar_num.run(dollar_num_acc { var: 0 }, tt2);
  assert_eq!(res4.var, 7);
  match dollar_num.run(dollar_num_acc { var: 0 }, tt.clone()) {
    (true, _, out_data) => {
      let mut tt3 = TokenStream::new();
      tt3.extend(TokenStream::from(TokenTree::Literal(Literal::u32_unsuffixed(out_data.var))));
      return tt3;
    },
    (false, x, _) => {
      let span = if let Some(s) = t.into_iter().next() {
        s.span()
      } else {
        Span::call_site()
      };
      let mut out_inner = TokenStream::new();
      out_inner.extend(x);
      out_inner.extend(compile_error("problem", span));
      let mut out = TokenStream::new();
      out.extend(TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace, out_inner))));
      return out;
    },
  }
}

fn compile_error(msg: &str, span: Span) -> TokenStream {
  let mut inner = TokenStream::new();
  inner.extend(TokenStream::from(TokenTree::Literal(Literal::string(msg))));
  let mut f = TokenStream::new();
  f.extend(TokenStream::from(TokenTree::Ident(Ident::new("compile_error", span))));
  f.extend(TokenStream::from(TokenTree::Punct(Punct::new('!', Spacing::Joint))));
  f.extend(TokenStream::from(TokenTree::Group(Group::new(Delimiter::Brace, inner))));
  f
}
