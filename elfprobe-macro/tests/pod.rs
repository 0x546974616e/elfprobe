#[test]
fn pod_derive() {
  use elfprobe_macro::Pod;

  #[derive(Pod)]
  pub(self) struct Dada;
  let _ = Dada;
}
