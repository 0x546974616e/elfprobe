///
/// Generate a tokens parser according to the given rules.
///
/// - **Syntax**:
///
///   - Repetitions:
///     - One or zero: `[A?]`
///     - One or more: `[A*]`
///     - Zero or more: `[A+]`
///   - Sequences: `(A B)`, `(A B C)`...
///   - Alternatives: `(A | B)`, `(A | B | C)`...
///   - Terminal: [identifier], [literal], [punctuation], [group]
///
/// [identifier]: proc_macro::Ident
/// [literal]: proc_macro::Literal
/// [punctuation]: proc_macro::Punct
/// [group]: proc_macro::Group
///
/// - **Notes**:
///
///   - Sequences (or [tuple]) and alternatives (or [union]) are currently
///     limited to 6 elements. To change it, see [`impl_collect_for_tuple`],
///     [`define_union`] and add a `#unpack_union()` rule to this macro.
///   - Repetitions must have parenthesis when including a non-terminal
///     (e.g. `[(A B)?]`).
///   - `[[A?]*]` causes an infinite loop, simply rewrite it as `[A*]`.
///
/// [union]: super::union::Union
/// [`define_union`]: super::union::define_union
/// [`impl_collect_for_tuple`]: super::collect::impl_collect_for_tuple
///
/// - **Example**:
///
/// The following definition
///
/// ```ignore
/// parser!(StructType = [OuterAttribute*] [Visibility?] (StructStruct | TupleStruct));
/// ```
///
/// will generate:
///
/// ```ignore
/// pub(crate) struct StructType {
///   pub tree: (
///     Vec<OuterAttribute>,
///     Option<Visibility>,
///     Union<StructStruct, TupleStruct>,
///   ),
/// }
///
/// impl Parse for StructType {
///   fn parse(input: Stream) -> Option<Self> {
///     // ...
///   }
/// }
/// ```
///
/// - **See** [`rules.rs`][super::rules] for more examples.
///
#[rustfmt::skip]
macro_rules! parser {

  // ╔╦╗┌─┐┌─┐┬  ┌─┐┬─┐┌─┐
  //  ║║├┤ │  │  ├─┤├┬┘├┤
  // ═╩╝└─┘└─┘┴─┘┴ ┴┴└─└─┘

  ($rule:ident = $($tt:tt)+) => {
    #[allow(unused)]
    #[allow(rustdoc::broken_intra_doc_links)]
    #[doc = stringify!($rule = $($tt)+)]
    pub(crate) struct $rule {
      pub(crate) tree: parser!(@type( ($($tt)+) )),
    }

    impl std::fmt::Debug for $rule {
      fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple(stringify!($rule)).field(&self.tree).finish()
      }
    }

    impl $crate::parser::collect::Collect for $rule {
      fn collect_into(&self, tokens: &mut Vec<proc_macro::TokenTree>) {
        self.tree.collect_into(tokens);
      }
    }

    impl $crate::parser::Parse for $rule {
      fn parse(input: $crate::parser::Stream) -> Option<Self> {
        { parser!(@parse( input, ($($tt)+) )) }.map(
          | tree | $rule { tree }
        )
      }
    }
  };

  // ╔╦╗┬ ┬┌─┐┌─┐
  //  ║ └┬┘├─┘├┤
  //  ╩  ┴ ┴  └─┘

  // Zero or more of TT.
  (@type( [$tt:tt*] )) => {
    Vec<parser!(@type( $tt ))>
  };

  // One or more of TT.
  (@type( [$tt:tt+] )) => {
    Vec<parser!(@type( $tt ))>
  };

  // Zero or one of TT.
  (@type( [$tt:tt?] )) => {
    Option<parser!(@type( $tt ))>
  };

  // Prevents unions of one element.
  // NOTE: "Sequence" cannot be before "Alternatives" because `tt` will eat all the pipes.
  (@type( ($tt:tt) )) => {
    parser!(@type( $tt ))
  };

  // Alternatives TT.
  (@type( ($($tt:tt)|+) )) => {
    $crate::parser::Union<$(parser!(@type( $tt )),)+>
  };

  // Sequence of TT.
  (@type( ($($tt:tt)+) )) => {
    ($(parser!(@type( $tt )),)+)
  };

  // Terminal TT.
  (@type( $tt:tt )) => {
    $tt
  };

  // ╔═╗┌─┐┬─┐┌─┐┌─┐
  // ╠═╝├─┤├┬┘└─┐├┤
  // ╩  ┴ ┴┴└─└─┘└─┘

  // Zero or more of TT.
  (@parse( $input:ident, [$tt:tt*] )) => {
    {
      let mut tokens = Vec::new();
      loop {
        match { parser!(@parse( $input, $tt )) } {
          Some(token) => tokens.push(token),
          None => break,
        }
      }
      Some(tokens)
    }
  };

  // One or more of TT.
  (@parse( $input:ident, [$tt:tt+] )) => {
    match { parser!(@parse( $input, [$tt*] )) } {
      Some(tokens) if tokens.is_empty() => None,
      value => value,
    }
  };

  // Zero or one of TT.
  (@parse( $input:ident, [$tt:tt?] )) => {
    Some({ parser!(@parse( $input, $tt )) })
  };

  // Prevents unions of one element.
  (@parse( $input:ident, ($tt:tt) )) => {
    parser!(@parse( $input, $tt ))
  };

  // Alternatives TT.
  (@parse( $input:ident, ($($tt:tt)|+) )) => {
    parser!(#unpack_union( $input, $($tt),+ ))
  };

  (#unpack_union( $input:ident, $a:tt )) => {
    parser!(#parse_union( $input, A($a) ))
  };

  (#unpack_union( $input:ident, $a:tt, $b:tt )) => {
    parser!(#parse_union( $input, A($a), B($b) ))
  };

  (#unpack_union( $input:ident, $a:tt, $b:tt, $c:tt )) => {
    parser!(#parse_union( $input, A($a), B($b), C($c) ))
  };

  (#unpack_union( $input:ident, $a:tt, $b:tt, $c:tt, $d:tt )) => {
    parser!(#parse_union( $input, A($a), B($b), C($c), D($d) ))
  };

  (#unpack_union( $input:ident, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt )) => {
    parser!(#parse_union( $input, A($a), B($b), C($c), D($d), E($e) ))
  };

  (#unpack_union( $input:ident, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt )) => {
    parser!(#parse_union( $input, A($a), B($b), C($c), D($d), E($e), F($f) ))
  };

  (#parse_union( $input:ident, $( $variant:tt($tt:tt) ),+ )) => {
    'alternate: {
      $(
        if let Some(value) = { parser!(@parse( $input, $tt )) } {
          break 'alternate Some($crate::parser::Union::$variant(value));
        }
      )+
      None
    }
  };

  // Sequence of TT.
  (@parse( $input:ident, ($($tt:tt)+) )) => {
    // try!() try{} (||{})()
    'sequence: {
      let behind = $input.fork();
      Some((
        $(
          match { parser!(@parse( $input, $tt )) } {
            Some(value) => value,
            None => {
              $input.merge(behind); // Reset cursor.
              break 'sequence None;
            }
          },
        )+
      ))
    }
  };

  // Terminal TT.
  (@parse( $input:ident, $tt:tt )) => {
    $input.parse::<$tt>()
  };

} // parser

pub(super) use parser;
