extern crate syntax;

use std::iter;
use syntax::codemap::{Pos, Spanned};

pub struct ParseNode<T> {
  pub children: Vec<Spanned<ParseNode<T>>>,
  pub value: T
}

pub fn annotate(spanned_node: Spanned<ParseNode<String>>, code: &str, indent: usize) -> String {
  let node = spanned_node.node;
  let span = spanned_node.span;
  let indentation = iter::repeat(" ").take(indent*4).collect::<String>() + "  ";
  format!("{}{}\n{}{} containing:\n\n{}",
    indentation,
    string_byte_range(code, span.lo.to_usize(), span.hi.to_usize()),
    indentation,
    node.value,
    node.children.into_iter().map(|node| annotate(node, code, indent+1)).collect::<Vec<String>>().join("\n")
  )
}

fn string_byte_range(source: &str, lo: usize, hi: usize) -> &str {
  &source[lo..hi]
}