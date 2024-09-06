#[test]
fn pod_derive() {
  use elfprobe_macro::Pod;

  #[derive(Pod)]
  struct Dada;
  let _ = Dada;
}
