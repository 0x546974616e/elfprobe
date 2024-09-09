use crate::cursor::Cursor;
use proc_macro::TokenTree;

pub(crate) type Stream<'buffer> = &'buffer Cursor<'buffer>;

pub(crate) trait Parse: Sized {
  // Parses and moves the cursor.
  fn parse(stream: Stream) -> Option<Self>;
}

pub(crate) trait Peek {
  // Checks required match, does not move the cursor.
  fn peek(stream: Stream) -> bool;
}

pub(crate) trait Collect {
  fn collect_into(&self, tree: &mut Vec<TokenTree>);
}

impl<Type: Collect> Collect for Option<Type> {
  fn collect_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(value) = self {
      value.collect_into(tree);
    }
  }
}

impl<Type: Collect> Collect for Vec<Type> {
  fn collect_into(&self, tree: &mut Vec<TokenTree>) {
    for value in self.iter() {
      value.collect_into(tree);
    }
  }
}

// ╦ ╦┌┐┌┬┌─┐┌┐┌
// ║ ║│││││ ││││
// ╚═╝┘└┘┴└─┘┘└┘

///
/// To increase the limit on sequences (`(A B C)`) and alternatives (`(A | B | C`),
/// modify the first macro rule below (with no parameters) and add as many
/// [`parser!(%parse(...))`][parser!()] rules as necessary.
///
#[rustfmt::skip]
macro_rules! define {
  // Just to mess up and try things out.
  // Completely unreadable and unnecessary (kind of).

  () => {
    // Union and Sequence are limited to 5 elements.
    // (Keep the same pattern, `A..E` and `4..0`, mandatory to work)
    define!(A.4, B.3, C.2, D.1, E.0);
  };

  ($($l:tt.$d:tt),+) => {
    define!(@collect $($l.$d)+);
    define!(@union $($l)+);
  };

  // ╔═╗┌─┐┬  ┬  ┌─┐┌─┐┌┬┐
  // ║  │ ││  │  ├┤ │   │
  // ╚═╝└─┘┴─┘┴─┘└─┘└─┘ ┴

  (@collect $tl:tt.$td:tt $($l:tt.$d:tt)*) => {
    define!(@collect/ $tl.$td $($l.$d)*);
    define!(@collect $($l.$d)*);
  };

  (@collect) => {
    impl Collect for () {
      fn collect_into(&self, _tree: &mut Vec<TokenTree>) {}
    }
  };

  (@collect/ $($l:tt.$d:tt)+) => {
    impl<$($l: Collect),+> Collect for ($($l,)+) {
      fn collect_into(&self, tree: &mut Vec<TokenTree>) {
        define!(#self tree [$($d)+] - []);
      }
    }
  };

  (#$self:tt $tree:tt [$h:tt $($t:tt)*] - [$($r:tt)*] ) => {
    define!(#$self $tree [$($t)*] - [$h$($r)*])
  };

  (#$self:tt $tree:tt [] - [$($d:tt)+]) => {
    $($self.$d.collect_into($tree);)*
  };

  // ╦ ╦┌┐┌┬┌─┐┌┐┌
  // ║ ║│││││ ││││
  // ╚═╝┘└┘┴└─┘┘└┘

  (@union $($l:tt)+) => {
    #[derive(Debug)]
    #[allow(unused)]
    pub(crate) enum Union<$($l: Collect = ()),+> {
      $($l($l)),+
    }

    impl<$($l: Collect),+> Collect for Union<$($l),+> {
      fn collect_into(&self, tree: &mut Vec<TokenTree>) {
        match self {
          $(Self::$l(value) => value.collect_into(tree)),+
        }
      }
    }
  };

} // define

define!();

// ╔═╗┌─┐┬─┐┌─┐┌─┐┬─┐
// ╠═╝├─┤├┬┘└─┐├┤ ├┬┘
// ╩  ┴ ┴┴└─└─┘└─┘┴└─

// https://doc.rust-lang.org/stable/unstable-book/language-features/try-blocks.html

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
///   - Sequences and alternatives are currently limited to 5 elements
///     (easy to change, see [`define!()`][define!()] macro for details).
///   - Repetitions must have parenthesis when including a non-terminal
///     (e.g. `[(A B)?]`).
///
/// - **Example**:
///
/// The following declaration,
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
/// - **See** [`rules.rs`][crate::rules] for more examples.
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
      pub tree: parser!(@type( ($($tt)+) )),
    }

    impl std::fmt::Debug for $rule {
      fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple(stringify!($rule)).field(&self.tree).finish()
      }
    }

    impl $crate::parser::Collect for $rule {
      fn collect_into(&self, tree: &mut Vec<proc_macro::TokenTree>) {
        self.tree.collect_into(tree);
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
      let mut tree = Vec::new();
      loop {
        match { parser!(@parse( $input, $tt )) } {
          Some(token) => tree.push(token),
          None => break,
        }
      }
      Some(tree)
    }
  };

  // One or more of TT.
  (@parse( $input:ident, [$tt:tt+] )) => {
    {
      match { parser!(@parse( $input, [$tt*] )) } {
        Some(tree) if tree.is_empty() => None,
        value => value,
      }
    }
  };

  // Zero or one of TT.
  (@parse( $input:ident, [$tt:tt?] )) => {
    {
      Some({ parser!(@parse( $input, $tt )) })
    }
  };

  // Prevents unions of one element.
  (@parse( $input:ident, ($tt:tt) )) => {
    parser!(@parse( $input, $tt ))
  };

  // Alternatives TT.
  (@parse( $input:ident, ($($tt:tt)|+) )) => {
    parser!(%parse( $input, $($tt),+ ))
  };

  (%parse( $input:ident, $a:tt )) => {
    parser!(#parse( $input, A($a) ))
  };

  (%parse( $input:ident, $a:tt, $b:tt )) => {
    parser!(#parse( $input, A($a), B($b) ))
  };

  (%parse( $input:ident, $a:tt, $b:tt, $c:tt )) => {
    parser!(#parse( $input, A($a), B($b), C($c) ))
  };

  (%parse( $input:ident, $a:tt, $b:tt, $c:tt, $d:tt )) => {
    parser!(#parse( $input, A($a), B($b), C($c), D($d) ))
  };

  (%parse( $input:ident, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt )) => {
    parser!(#parse( $input, A($a), B($b), C($c), D($d), E($e) ))
  };

  (#parse( $input:ident, $( $variant:tt($tt:tt) ),+ )) => {
    {
      if false { None }
      $(
        else if let Some(value) = { parser!(@parse( $input, $tt )) } {
          Some($crate::parser::Union::$variant(value))
        }
      )+
      else { None }
    }
  };

  // Sequence of TT.
  (@parse( $input:ident, ($($tt:tt)+) )) => {
    {
      let behind = $input.fork();
      (|| { // try!() try{}
        Some((
          $(
            match { parser!(@parse( $input, $tt )) } {
              Some(value) => value,
              None => {
                $input.merge(behind);
                return None;
              }
            },
          )+
        ))
      })()
    }
  };

  // Terminal TT.
  (@parse( $input:ident, $tt:tt )) => {
    $input.parse::<$tt>()
  };

} // parser

pub(crate) use parser;
