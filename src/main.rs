#![allow(dead_code)]

mod highlight_error;

fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("{}", highlight_error::highlight_error(38, 64, &code));
}
