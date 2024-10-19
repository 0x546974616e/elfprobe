use std::fmt;

#[derive(Eq, PartialEq)]
pub struct Constant<Type> {
  name: Option<&'static str>,
  value: Type, // Integer value.
  meaning: Option<&'static str>,
}

impl<Type> Constant<Type> {
  #[inline(always)]
  pub fn new(name: Option<&'static str>, value: Type, meaning: Option<&'static str>) -> Self {
    Self { name, value, meaning }
  }

  #[inline(always)]
  pub fn named(name: &'static str, value: Type, meaning: Option<&'static str>) -> Self {
    Self::new(Some(name), value, meaning)
  }

  #[inline(always)]
  pub fn unknown(value: Type, meaning: Option<&'static str>) -> Self {
    Self::new(None, value, meaning)
  }

  #[inline(always)]
  pub fn name(&self) -> &'static str {
    self.name.unwrap_or("Unknown")
  }

  #[inline(always)]
  pub fn meaning(&self) -> &'static str {
    self.meaning.unwrap_or("") // "??"
  }
}

impl<Type: fmt::Debug + fmt::LowerHex> fmt::Debug for Constant<Type> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_tuple(self.name())
      .field(&self.value)
      .field(&self.meaning())
      .finish()
  }
}

impl<Type: fmt::Display + fmt::LowerHex> fmt::Display for Constant<Type> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if formatter.alternate() {
      if let Some(meaning) = self.meaning {
        return match self.name {
          Some(name) => formatter.write_fmt(format_args!("{} ({})", name, meaning)),
          None => formatter.write_fmt(format_args!("{:#x} ({})", self.value, meaning)),
        };
      }
    }

    match self.name {
      Some(name) => formatter.write_str(name),
      // None => formatter.write_fmt(format_args!("{:#x}", self.value)),
      None => formatter.write_fmt(format_args!("Unknown ({:#x})", self.value)),
      // None => fmt::LowerHex::fmt(&self.value, formatter),
    }
  }
}

impl<Type: fmt::Display + fmt::LowerHex> Constant<Type> {
  #[allow(unused)]
  #[inline(always)]
  pub fn name_or_value(&self) -> String {
    self.to_string()
  }
}

macro_rules! define_constants {
  (
    $module:ident($type:ty) $description:literal,
    $( $name1:ident = $value1:literal $meaning1:literal, )*
    $( [$name2:ident, $name3:ident] = [$value2:literal, $value3:literal] $meaning2:literal, )*
  ) => {
    #[allow(unused)]
    #[doc = $description]
    pub mod $module {
      use $crate::utils::Constant;

      $(
        #[allow(unused)]
        #[doc = $meaning1]
        pub const $name1: $type = $value1;
      )*

      $(
        #[allow(unused)]
        #[doc = $meaning2]
        pub const $name2: $type = $value2;

        #[allow(unused)]
        #[doc = $meaning2]
        pub const $name3: $type = $value3;
      )*

      #[allow(unused)]
      #[doc = concat!("Transforms an `", stringify!($type), "` into an [`", stringify!($module), "`][self] constant.")]
      pub fn into_constant(value: impl Into<$type>) -> Constant<$type> {
        let value = value.into();
        match value {
          $( $value1 => Constant::named(stringify!($name1), value, Some($meaning1)), )*

          $(
            $value2 => Constant::named(stringify!($name2), value, Some($meaning2)),
            $value3 => Constant::named(stringify!($name3), value, Some($meaning2)),
            $value2 .. $value3 => Constant::unknown(value, Some($meaning2)),
          )*

          _ => Constant::unknown(value, None),
        }
      }
    }
  };
}

pub(crate) use define_constants;

#[cfg(test)]
mod tests {
  use super::*;

  define_constants! {
    dada(usize) "Dada",

    TR_DADA = 0 "Dada fafa",
    TR_FAFA = 1 "Fafa gaga",
    TR_GAGA = 101 "Gaga haha",

    [ TR_LOHAHA, TR_HIHAHA ] = [ 0xA0, 0xA8 ] "Haha jaja",
    [ TR_LOJAJA, TR_HIJAJA ] = [ 0xB0, 0xBC ] "Jaja kaka",
  }

  #[test]
  fn named_dada() {
    assert_eq!(
      Constant::<usize>::named("TR_DADA", 0, Some("Dada fafa")),
      dada::into_constant(0_usize),
    );
  }

  #[test]
  fn named_fafa() {
    assert_eq!(
      Constant::<usize>::named("TR_FAFA", 1, Some("Fafa gaga")),
      dada::into_constant(1_usize),
    );
  }

  #[test]
  fn named_gaga() {
    assert_eq!(
      Constant::<usize>::named("TR_GAGA", 101, Some("Gaga haha")),
      dada::into_constant(dada::TR_GAGA),
    );
  }

  #[test]
  fn named_lohaha() {
    assert_eq!(
      Constant::<usize>::named("TR_LOHAHA", 0xA0, Some("Haha jaja")),
      dada::into_constant(0xA0_usize),
    );
  }

  #[test]
  fn named_hijaja() {
    assert_eq!(
      Constant::<usize>::named("TR_HIJAJA", 0xBC, Some("Jaja kaka")),
      dada::into_constant(0xBC_usize),
    );
  }

  #[test]
  fn unknown_haha() {
    assert_eq!(
      Constant::<usize>::unknown(0xA5, Some("Haha jaja")),
      dada::into_constant(0xA5_usize),
    );
  }

  #[test]
  fn unknonw_jaja() {
    assert_eq!(
      Constant::<usize>::unknown(0xBB, Some("Jaja kaka")),
      dada::into_constant(0xBB_usize),
    );
  }

  #[test]
  fn unknown() {
    assert_eq!(
      Constant::<usize>::unknown(0x3, None),
      dada::into_constant(0x3_usize),
    );
  }
}
