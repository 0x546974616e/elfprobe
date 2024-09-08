trait La {}
trait Do {}

#[test]
fn pod_derive() {
  use elfprobe_macro::Pod;

  trait Fafa {}
  mod a {
    pub trait Ici {}
    pub mod b {
      pub mod c {
        pub mod d {
          pub trait Gaga {}
        }
      }
    }
  }

  #[derive(Pod)]
  #[allow(unused)]
  #[repr(packed)]
  pub(self) struct Dada<
    'dada: 'static + 'static,
    'a: 'static + 'dada + 'static,
    T: 'static + self::La + 'dada + 'a + Fafa + a::b::c::d::Gaga + crate::Do,
    B: self::Do,
    E:,
    F: a::Ici +,
  >(&'dada B, &'a T, E, F);
  // let _ = Dada;
}
