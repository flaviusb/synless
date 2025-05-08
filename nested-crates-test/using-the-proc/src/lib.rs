extern crate synless;
extern crate the_proc;

use synless::*;
use std::rc::Rc;

use the_proc::*;

#[test]
pub fn test_dollar() {
  let x = dollar_increment!($1);
  let y = dollar_increment!($ 2);
  assert_eq!(x, 1);
  assert_eq!(y, 2);
  //let z = dollar_increment!($ $);
}


