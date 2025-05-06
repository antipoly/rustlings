// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
  Uppercase,
  Trim,
  Append(usize),
}

mod my_module {
  use super::Command;

  // TODO: Complete the function as described above.
  pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    let mut output = Vec::new();
    let mut num = 0;

    for (string, command) in input.iter() {
      let transformed_str;

      match command {
        Command::Trim => transformed_str = string.trim().to_string(),
        Command::Uppercase => transformed_str = string.to_uppercase().to_string(),
        Command::Append(n) => {
          let mut cloned = string.to_owned();
          for _ in 0..*n {
            cloned.push_str("bar");
          }

          transformed_str = cloned;
        }
      }

      output.insert(num, transformed_str.to_string());
      num += 1;
    }

    output
  }
}

fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  // TODO: What do we need to import to have `transformer` in scope?
  use super::my_module::transformer;
  use super::Command;

  #[test]
  fn it_works() {
    let input = vec![
      ("hello".to_string(), Command::Uppercase),
      (" all roads lead to rome! ".to_string(), Command::Trim),
      ("foo".to_string(), Command::Append(1)),
      ("bar".to_string(), Command::Append(5)),
    ];
    let output = transformer(input);

    assert_eq!(
      output,
      [
        "HELLO",
        "all roads lead to rome!",
        "foobar",
        "barbarbarbarbarbar",
      ]
    );
  }
}
