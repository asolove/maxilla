#![feature(rustc_private)]
extern crate syntax;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_plugin;

use rustc::middle::cstore::DummyCrateStore;
use rustc::session::{self,config};
use rustc::session::config::Input;
use rustc_driver::driver;

use std::rc::Rc;
use syntax::ast;

mod explain;
use explain::{SpannedExplainParse};
mod tree;


fn main() {
    let code = "fn main() { let greeting = greet(\"Dave\"); print!(greeing); }\nfn greet(name: &str) -> &str { \"Hello, \" + name }";
    let krate = parse_code(code);
    let explanation = krate.module.spanned_explain();
    println!("{}", tree::annotate(explanation, code, 0));
}

fn parse_code(code: &str) -> ast::Crate {
    let options = config::basic_options();
    let session = session::build_session(options, None,
                                         syntax::diagnostics::registry::Registry::new(&[]),
                                         Rc::new(DummyCrateStore));
    let cfg: ast::CrateConfig = vec![];
    let input = Input::Str(code.to_string());
    driver::phase_1_parse_input(&session, cfg, &input)
}