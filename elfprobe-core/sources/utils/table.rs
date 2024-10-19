use std::cmp::max;
use std::fmt::{self, Write};

enum TableItem {
  Value(String),
  NewLine,
}

pub struct TableBuilder<'formatter, 'buffer: 'formatter> {
  formatter: &'formatter mut fmt::Formatter<'buffer>,
  result: fmt::Result,
  items: Vec<TableItem>,
  widths: Vec<usize>,
}

impl<'formatter, 'buffer: 'formatter> TableBuilder<'formatter, 'buffer> {
  pub(self) fn set_width(&mut self, index: usize, width: usize) {
    match self.widths.get(index) {
      Some(current) => self.widths[index] = max(*current, width),
      None => self.widths.push(width),
    }
  }

  pub(self) fn get_width(&self, index: usize) -> usize {
    *self.widths.get(index).unwrap_or(&0)
  }

  /// Values should not contain new lines.
  pub fn row(&mut self, values: &[&dyn fmt::Display]) -> &mut Self {
    self.items.push(TableItem::NewLine);

    // TODO: Could be better handled.
    for (index, value) in values.iter().enumerate() {
      // See rustlib/src/rust/library/core/src/fmt/builders.rs
      self.result = self.result.and_then(|_| {
        let mut string = String::new();

        // error[E0658]: use of unstable library feature 'fmt_internals': internal to standard library
        // | let mut buffer = String::new();
        // | let mut formatter = fmt::Formatter::new(&mut buffer);
        // |                     ^^^^^^^^^^^^^^^^^^^
        // | value.fmt(&mut formatter)

        if self.formatter.alternate() {
          write!(&mut string, "{:#}", value)?;
        } else {
          write!(&mut string, "{}", value)?;
        }

        self.set_width(index, string.len());
        self.items.push(TableItem::Value(string));

        Ok(())
      });
    }

    self
  }

  pub fn finish(&mut self) -> fmt::Result {
    self.result?;
    let mut index = 0usize;

    for item in self.items.iter() {
      match item {
        TableItem::NewLine => index = 0,
        TableItem::Value(value) => {
          if index == 0 {
            self.formatter.write_str("\n  ")?;
          }

          let width = self.get_width(index);
          self.formatter.write_fmt(format_args!("{value:<width$} "))?;
          index += 1;
        }
      }
    }

    self.formatter.write_char('\n')
  }
}

pub trait DisplayTable<'formatter, 'buffer: 'formatter> {
  fn display_table(&'formatter mut self, title: &str) -> TableBuilder<'formatter, 'buffer>;
}

impl<'formatter, 'buffer: 'formatter> DisplayTable<'formatter, 'buffer> for fmt::Formatter<'buffer> {
  fn display_table(&'formatter mut self, title: &str) -> TableBuilder<'formatter, 'buffer> {
    let result = self.write_str(title);
    TableBuilder {
      result,
      formatter: self,
      items: Vec::new(),
      widths: Vec::new(),
    }
  }
}

macro_rules! display_table {
  (
    $formatter: ident, $title: expr =>
    $( [ $( $item: expr ),* ] ),* $(,)?
  ) => {
    {
      use $crate::utils::DisplayTable;
      let mut table = $formatter.display_table($title);
      $( table.row( &[ $( &$item ),* ] ); )*
      table.finish()
    }
  };
}

pub(crate) use display_table;

#[cfg(test)]
mod tests {
  use super::*;

  struct Dada {
    a: isize,
  }

  impl fmt::Display for Dada {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      formatter.debug_struct("Dada").field("a", &self.a).finish()
    }
  }

  struct Fafa {
    b: usize,
  }

  impl fmt::Display for Fafa {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      let answer = 42;
      display_table!(
        formatter, "Fafa" =>
        [ Dada { a: -1 }, false, "foobar" ],
        [ &answer, true, Dada { a: 101 } ],
        [ "Looooooooooooooooong", self.b ],
        [ &1, &2, &3, &4, &5, &6 ],
      )
    }
  }

  #[test]
  fn display_table() {
    assert_eq!(
      Fafa { b: 123 }.to_string(),
      concat!(
        "Fafa\n",
        "  Dada { a: -1 }       false foobar          \n",
        "  42                   true  Dada { a: 101 } \n",
        "  Looooooooooooooooong 123   \n",
        "  1                    2     3               4 5 6 \n",
      )
    )
  }
}
