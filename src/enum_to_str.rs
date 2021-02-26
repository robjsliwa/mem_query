#[macro_export]
macro_rules! enum_to_str {
    ($(#[$m:meta])* $vis:vis enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
      $(#[$m])*
      $vis enum $name {
          $($variant = $val),*,
      }
      impl $name {
          $vis fn name(&self) -> &'static str {
              match self {
                  $($name::$variant => stringify!($variant)),*
              }
          }
          $vis fn lowercase_name(&self) -> &'static str {
            match self {
              $($name::$variant => paste! { stringify!([<$variant:lower>])}),*
            }
          }
          $vis fn as_operator(&self) -> &'static str {
            match self {
              $($name::$variant => paste! { concat! ("$", stringify!([<$variant:lower>]))}),*
            }
          }
      }
    };
    ($(#[$m:meta])* $vis:vis enum $name:ident {
        $($variant:ident),*,
    }) => {
      $(#[$m])*
      $vis enum $name {
          $($variant),*,
      }
      impl $name {
          $vis fn name(&self) -> &'static str {
              match self {
                  $($name::$variant => stringify!($variant)),*
              }
          }
          $vis fn lowercase_name(&self) -> &'static str {
            match self {
                $($name::$variant => paste! { stringify!([<$variant:lower>])}),*
            }
          }
          $vis fn as_operator(&self) -> &'static str {
            match self {
              $($name::$variant => paste! { concat! ("$", stringify!([<$variant:lower>]))}),*
            }
          }
      }
    };
}

#[cfg(test)]
mod tests {
  #[test]
  fn simple_enum() {
    enum_to_str! {
      enum SimpleTest {
        ONE,
        TWO,
      }
    }
    assert_eq!(SimpleTest::ONE.name(), "ONE");
    assert_eq!(SimpleTest::TWO.name(), "TWO");
    assert_eq!(SimpleTest::ONE.lowercase_name(), "one");
    assert_eq!(SimpleTest::TWO.lowercase_name(), "two");
    assert_eq!(SimpleTest::ONE.as_operator(), "$one");
    assert_eq!(SimpleTest::TWO.as_operator(), "$two");
  }
  #[test]
  fn simple_and_meta_enum() {
    enum_to_str! {
      #[derive(Debug)]
      enum SimpleTest {
        ONE,
        TWO,
      }
    }
    assert_eq!(SimpleTest::ONE.name(), "ONE");
    assert_eq!(SimpleTest::TWO.name(), "TWO");
    assert_eq!(SimpleTest::ONE.lowercase_name(), "one");
    assert_eq!(SimpleTest::TWO.lowercase_name(), "two");
    assert_eq!(SimpleTest::ONE.as_operator(), "$one");
    assert_eq!(SimpleTest::TWO.as_operator(), "$two");
  }
  #[test]
  fn with_assignment_enum() {
    enum_to_str! {
      enum WithAssignTest {
        ONE = 1,
        TWO = 2,
      }
    }
    assert_eq!(WithAssignTest::ONE.name(), "ONE");
    assert_eq!(WithAssignTest::TWO.name(), "TWO");
    assert_eq!(WithAssignTest::ONE.lowercase_name(), "one");
    assert_eq!(WithAssignTest::TWO.lowercase_name(), "two");
    assert_eq!(WithAssignTest::ONE.as_operator(), "$one");
    assert_eq!(WithAssignTest::TWO.as_operator(), "$two");
  }
  #[test]
  fn with_assignment_and_meta_enum() {
    enum_to_str! {
      #[derive(Debug)]
      enum AssignAndMetaTest {
        ONE = 1,
        TWO = 2,
      }
    }
    assert_eq!(AssignAndMetaTest::ONE.name(), "ONE");
    assert_eq!(AssignAndMetaTest::TWO.name(), "TWO");
    assert_eq!(AssignAndMetaTest::ONE.lowercase_name(), "one");
    assert_eq!(AssignAndMetaTest::TWO.lowercase_name(), "two");
    assert_eq!(AssignAndMetaTest::ONE.as_operator(), "$one");
    assert_eq!(AssignAndMetaTest::TWO.as_operator(), "$two");
  }
}
