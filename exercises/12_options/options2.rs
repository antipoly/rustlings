fn main() {
  // You can optionally experiment here.

  // Option<> is an enum with a value of either None or Some(T))
  // Some(T) represents a value of type T that is present wrapped in a special object
  // None represents the absence of a value. It is used when there is no meaningful value or when something is optional.

  // if let Some(value) = some_value {}
  // Used to match and destructure an Option value. If the value is Some, the control flow enters the block, and you can access the inner value.
  // If the value is None, the block is skipped

  // while let Some(value) = some_value {}
  // It repeatedly matches and destructures an Option value in a loop. The loop continues as long as the value is Some. If the value is None, the loop exits.
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_option() {
    let target = "rustlings";
    let optional_target = Some(target);

    // TODO: Make this an if-let statement whose value is `Some`.
    if let Some(word) = optional_target {
      assert_eq!(word, target);
    }
  }

  #[test]
  fn layered_option() {
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
      optional_integers.push(Some(i));
    }

    let mut cursor = range;

    // TODO: Make this a while-let statement. Remember that `Vec::pop()`
    // adds another layer of `Option`. You can do nested pattern matching
    // in if-let and while-let statements.
    while let Some(integer) = optional_integers.pop() {
      if let Some(i) = integer {
        assert_eq!(i, cursor);
        cursor -= 1;
      }
    }

    assert_eq!(cursor, 0);
  }
}
