#[test]
fn pod_derive() {
  use elfprobe_macro::Pod;

  #[derive(Pod)]
  #[allow(unused)]
  #[repr(packed)]
  pub(self) struct Dada<'dada: 'static + 'static +, 'a: 'static + 'dada + 'static +, T>(&'dada u8, &'a T);
  // let _ = Dada;
}
