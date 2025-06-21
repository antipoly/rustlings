// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

fn main() {
  // TODO: Fix the compiler error by moving one line.

  let string1 = String::from("long string is long");
  let result;
  
  // NOTE: Any input which is borrowed must live longer than its borrower
  // The borrowed value 'result' lives longer in scope than the input 'string2'
  // So it was moved outside the scope
  let string2 = String::from("xyz");
  {
    result = longest(&string1, &string2);
  }
  println!("The longest string is '{result}'");
}
