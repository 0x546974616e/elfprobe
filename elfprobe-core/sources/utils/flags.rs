
macro_rules! define_flags {
  (
    $struct:ident($type:ty) $description:literal,
    $( $name:ident $(/ $short:ident)? = $value:literal $meaning:literal, )*
  ) => {
    $(
      #[doc = $meaning]
      #[allow(unused, non_upper_case_globals)]
      pub const $name: $type = (1 << $value);
    )*

    #[repr(transparent)]
    pub struct $struct {
      flags: $type,
    }

    impl From<$type> for $struct {
      #[inline(always)]
      fn from(flags: $type) -> Self {
        Self { flags }
      }
    }

    impl std::fmt::Display for $struct {
      fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let $struct { mut flags } = self;

        $(
          if flags & $name != 0 {
            if flags != self.flags {
              formatter.write_char(' ')?;
            }

            formatter.write_str(stringify!($name))?;
            flags &= !$name;
          }
        )*

        if flags != 0 {
          if flags != self.flags {
            formatter.write_char(' ')?;
          }

          formatter.write_fmt(format_args!( "0b{:b}", flags))?;
        }

        Ok(())
      }
    }
  };
}

pub(crate) use define_flags;

#[cfg(test)]
mod tests {
  use super::define_flags;

  define_flags!{
    Dada(usize) "dada",

    TR_DADA = 1 "Dada fafa",
    TR_FAFA / F = 2 "Fafa gaga",
  }

  #[test]
  fn dada() {
    let dada = Dada::from(TR_DADA | TR_FAFA | 0b100000);
    print!("{}", dada);
  }
}
