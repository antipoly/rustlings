trait Licensed {
  fn licensing_info(&self) -> String {
    "Default license".to_string()
  }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
// NOTE: impl {trait} is shorthand for "any type that implements the {trait}"
// The same can be applied to the return type as well
// https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
  software1.licensing_info() == software2.licensing_info()
}

// NOTE: The trait parameter syntax above is syntactical sugar for the trait bound syntax below
// In this example now: all parameters of T must have the same type
// https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax
fn compare_license_types_v2<T: Licensed>(software1: T, software2: T) -> bool {
  software1.licensing_info() == software2.licensing_info()
}

fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn compare_license_information() {
    assert!(compare_license_types(SomeSoftware, OtherSoftware));
  }

  #[test]
  fn compare_license_information_backwards() {
    assert!(compare_license_types(OtherSoftware, SomeSoftware));
  }
}
