#[macro_use]
extern crate paste;
mod enum_to_str;

enum_to_str! {
  pub enum ComparisonOperators {
    EQ,
    GT,
    GTE,
    LT,
    LTE,
    NE,
    IN,
    NIN,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_operator_generation() {
    assert_eq!(ComparisonOperators::EQ.as_operator(), "$eq");
  }
}
