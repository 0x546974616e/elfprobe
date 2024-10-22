use proc_macro::TokenTree;

pub(crate) trait Collect {
  fn collect_into(&self, tokens: &mut Vec<TokenTree>);
}

impl<Type: Collect> Collect for Option<Type> {
  fn collect_into(&self, tokens: &mut Vec<TokenTree>) {
    if let Some(value) = self {
      value.collect_into(tokens);
    }
  }
}

impl<Type: Collect> Collect for Vec<Type> {
  fn collect_into(&self, tokens: &mut Vec<TokenTree>) {
    for value in self.iter() {
      value.collect_into(tokens);
    }
  }
}

/// Implements Collect for 0-tuple to 6-tuple.
macro_rules! impl_collect_for_tuple {
  () => {
    // From ()-tuple to (A, B, C, D, E, F)-tuple.
    impl_collect_for_tuple!(@call A B C D E F);
  };

  (@call) => { // Terminating condition.
    impl_collect_for_tuple!(@impl);
  };

  (@call $head:ident $($name:ident)*) => {
    impl_collect_for_tuple!(@call $($name)*);
    impl_collect_for_tuple!(@impl $head $($name)*);
  };

  (@impl $($name:ident)*) => {
    impl<$($name: Collect,)*> Collect for ($($name,)*) {
      #[allow(unused_variables)]
      fn collect_into(&self, tokens: &mut Vec<TokenTree>) {
        #[allow(non_snake_case)]
        let ($($name,)*) = self;
        $($name.collect_into(tokens);)*
      }
    }
  };
}

#[cfg(doc)]
pub(super) use impl_collect_for_tuple;

impl_collect_for_tuple!();
