use proc_macro::TokenTree;

use super::collect::Collect;

/// Defines an union of 6 elements.
macro_rules! define_union {
  () => {
    // Defines an Union<A, B, C, D, E, F>.
    define_union!(@impl A B C D E F);
  };

  (@impl $($name:ident)+) => {
    #[derive(Debug)]
    pub(crate) enum Union<$($name: Collect = ()),+> {
      $(#[allow(unused)] $name($name)),+
    }

    impl<$($name: Collect,)+> Collect for Union<$($name,)+> {
      fn collect_into(&self, token: &mut Vec<TokenTree>) {
        match self {
          $(Self::$name(value) => value.collect_into(token)),+
        }
      }
    }
  };
}

#[cfg(doc)]
pub(super) use define_union;

define_union!();
