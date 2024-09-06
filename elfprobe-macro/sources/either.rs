#![allow(unused)]

// Like in Haskell:
// data Either a b = Left a | Right b
pub enum Either<A, B> {
  Left(A),
  Right(B),
}

impl<A, B> Either<A, B> {
  pub fn unwrap_left(self) -> A {
    match self {
      Either::Left(left) => left,
      Either::Right(_right) => {
        panic!("Called Either::unwrap_left() on a Right value.")
      }
    }
  }

  pub fn unwrap_right(self) -> B {
    match self {
      Either::Right(right) => right,
      Either::Left(_left) => {
        panic!("Called Either::unwrap_right() on a Left value.")
      }
    }
  }
}
