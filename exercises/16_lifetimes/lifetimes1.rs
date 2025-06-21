// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// NOTE: Any input which is borrowed must live longer than its borrower
// Translation: Any variable which has a reference made of it, must outlive all of its references in scope, else it would risk the references pointing to a null value

// TODO: Fix the compiler error by updating the function signature.
// NOTE: Translation: x, y and the return value MUST live for at least as long as lifetime parameter 'a
// The lifetime is determined by the shortest living variable
// 'a is a variable just like a generic and can have a unique name
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_longest() {
    assert_eq!(longest("abcd", "123"), "abcd");
    assert_eq!(longest("abc", "1234"), "1234");
  }
}
