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
use syntax::ast::{self, Item_, Stmt_};


fn main() {
    let code = "fn main() { let a : &'static str = \"Hello\" + \", world!\"; println!(a); }";
    println!("Parsing code: {}", code);
    let krate = parse_code(code);

    match krate.module.items[0].node {
        Item_::ItemFn(ref _decl, _, _, _, ref _generics, ref block) =>
            match block.stmts[0].node {
                Stmt_::StmtDecl(ref decl, _id) => println!("Parsed let statement: {:?}", decl.node),
                _ => unreachable!()
            },
        _ => unreachable!()
    }
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
