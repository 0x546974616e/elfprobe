
#[cfg(test)]
mod tests {
  use elfprobe_macro::is_bin_literal;
  use elfprobe_macro::is_hex_literal;

  #[test]
  fn is_hex_literal() {
    let result: bool = is_hex_literal!(0x12aB);
    assert_eq!(result, true);
  }

  #[test]
  fn is_not_hex_literal() {
    let result: bool = is_hex_literal!(112);
    assert_eq!(result, false);
  }

  #[test]
  fn is_bin_literal() {
    let result: bool = is_bin_literal!(0b10110);
    assert_eq!(result, true);
  }

  #[test]
  fn is_not_bin_literal() {
    let result: bool = is_bin_literal!(0x123);
    assert_eq!(result, false);
  }
}
