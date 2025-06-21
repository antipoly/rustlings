use std::fmt::{Debug, Display};

trait SomeTrait {
  fn some_function(&self) -> bool {
    true
  }
}

trait OtherTrait {
  fn other_function(&self) -> bool {
    true
  }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
// NOTE: https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
  item.some_function() && item.other_function()
}

// NOTE: where clauses allow generics to be less cluttered
// https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses
fn some_func_v2<T, U>(t: &T, u: &U) -> bool
where
  T: Display + Clone,
  U: Clone + Debug
{
  true
}

fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_some_func() {
    assert!(some_func(SomeStruct));
    assert!(some_func(OtherStruct));
  }
}
