use crate::entry::Identifier;
use crate::parser::parser;
use crate::token::*;

// ╦═╗┬ ┬┬  ┌─┐┌─┐
// ╠╦╝│ ││  ├┤ └─┐
// ╩╚═└─┘┴─┘└─┘└─┘

// https://doc.rust-lang.org/reference/items/structs.html#structs
parser!(StructType = [OuterAttribute*] [Visibility?] (StructStruct | TupleStruct));
parser!(StructStruct = Struct Identifier [GenericParams?] (Brace | SemiColon));
parser!(TupleStruct = Struct Identifier [GenericParams?] Parenthesis SemiColon);

// https://doc.rust-lang.org/reference/attributes.html
parser!(OuterAttribute = Hash Bracket );

// https://doc.rust-lang.org/reference/visibility-and-privacy.html
parser!(Visibility = Pub[Parenthesis?]);

// https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
parser!(GenericParams = Lt [(GenericParam [Comma?])*] Gt);
parser!(GenericParam = [OuterAttribute*] (LifetimeParam | TypeParam | ConstParam));
parser!(LifetimeParam = Lifetime [(Colon LifetimeBounds)?]);
parser!(TypeParam = Identifier [(Colon TypeParamBounds)?]); // TODO: "= Type"
parser!(ConstParam = Const Identifier Colon Identifier); // TODO: ": Type (= Block | Identifier | Literal)?"

// https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels
parser!(Lifetime = Quote Identifier); // LifetimeOrLabel

// https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
parser!(TypeParamBounds = [([Plus?] TypeParamBound)*]);
parser!(TypeParamBound = (Lifetime | TraitBound));
parser!(TraitBound = (Parenthesis | TypePath)); // TODO: "? ForLifetimes"
parser!(LifetimeBounds = [(Lifetime [Plus?])*]);

// https://doc.rust-lang.org/reference/paths.html#paths-in-types
parser!(DoubleColon = Colon Colon);
parser!(TypePath = [([DoubleColon?] Identifier)+]); // TODO: "TypePathSegment" instead of "Identifier"
parser!(TypePathSegment = PathIdentSegment); // TODO: ":: (GenericArgs | TypePathFn)"

// https://doc.rust-lang.org/reference/paths.html#paths-in-expressions
parser!(PathIdentSegment = Identifier); // TODO: "$crate"

// ╔═╗┬ ┬┌─┐┌┬┐┌─┐┌┬┐
// ║  │ │└─┐ │ │ ││││
// ╚═╝└─┘└─┘ ┴ └─┘┴ ┴

impl StructStruct {
  fn collect_impl() {

  }

  fn collect_types() {

  }
}
