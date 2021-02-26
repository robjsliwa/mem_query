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
