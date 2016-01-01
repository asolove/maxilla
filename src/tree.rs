extern crate syntax;

use syntax::codemap::{Pos, Span, Spanned};

pub struct ParseNode<T> {
  pub children: Vec<Spanned<ParseNode<T>>>,
  pub value: T
}

pub fn annotate(spanned_node: Spanned<ParseNode<String>>, code: &str, indent: u8) -> String {
  let node = spanned_node.node;
  let span = spanned_node.span;
  format!("{}{}\n{}{} containing {}",
    indent,
    string_byte_range(code, span.lo.to_usize(), span.hi.to_usize()),
    indent,
    node.value,
    node.children.into_iter().map(|node| annotate(node, code, indent+1)).collect::<Vec<String>>().join("\n")
  )
}

fn string_byte_range(source: &str, lo: usize, hi: usize) -> &str {
  &source[lo..hi]
}