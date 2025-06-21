// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

// Clippy wants you to fix several lints in this file:
// 1. Unwrapping a None value will panic. Use pattern matching or unwrap_or/unwrap_or_else.
// 2. The array is missing commas between elements.
// 3. The result of `resize` is (), not a Vec. You probably want to create a Vec and then call resize on it, or use Vec::new().
// 4. Swapping values by assignment doesn't actually swap; use std::mem::swap or std::mem::replace.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
  let my_option: Option<()> = None;
  if let Some(opt) = my_option {
    println!("{opt:?}");
  }

  let my_arr = &[
    -1, -2, -3,
    -4, -5, -6
  ];
  println!("My array! Here it is: {my_arr:?}");

  let mut my_empty_vec = vec![1, 2, 3, 4, 5];
  my_empty_vec.clear();
  println!("This Vec is empty, see? {my_empty_vec:?}");

  let mut value_a = 45;
  let mut value_b = 66;
  // Let's swap these two!
  std::mem::swap(&mut value_a, &mut value_b);
  println!("value a: {value_a}; value b: {value_b}");
}
