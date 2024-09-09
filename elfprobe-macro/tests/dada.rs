#[test]
fn dadafafagaga() {
  use elfprobe_macro::test_macro_parser;
  // test_dada!(pub = where for # # = ; ; ; # for pub for pub for pub # + ;);
  test_macro_parser!(
    #[derive(Dada)]
    #[allow(unused)]
    pub(self) struct Dada<'a: 'dada + 'fafa +, A: + pub::pub pub + pub>();
  );
}
