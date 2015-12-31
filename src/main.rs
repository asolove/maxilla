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
use syntax::ast::{self, Item_, Stmt_, Decl_, Pat_, Ty_, Expr_};


fn main() {
    let code = "fn main() { let a : &'static str = \"Hello\" + \", world!\"; println!(a); }";
    println!("Parsing the first statement inside this function declaration: {}", code);
    let krate = parse_code(code);

    match krate.module.items[0].node {
        Item_::ItemFn(ref _decl, _, _, _, ref _generics, ref block) =>
            println!("{}", block.stmts[0].node.explain()),
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

trait ExplainParse {
    fn explain(&self) -> String;
}

impl ExplainParse for Stmt_ {
    fn explain(&self) -> String {
        match *self {
            Stmt_::StmtDecl(ref decl, _id) => format!("A declaration of {}", decl.node.explain()),
            _ => "some other statement".to_string()
        }
    }
}

impl ExplainParse for Decl_ {
    fn explain(&self) -> String {
        match *self {
            Decl_::DeclLocal(ref local) => format!("a local assigning {} of type {:?} to {:?}",
                                                local.pat.node.explain(),
                                                local.ty.as_ref().map_or_else(|| "undeclared".to_string(), |ty| ty.node.explain()),
                                                local.init.as_ref().map_or_else(|| "uninitialized".to_string(), |init| init.node.explain())),
            Decl_::DeclItem(ref _item) => "item declaration?".to_string()
        }
    }
}

impl ExplainParse for Pat_ {
    fn explain(&self) -> String {
        match *self {
            Pat_::PatIdent(_, ref ident, _) => format!("the symbol {}", ident.node.name.as_str()),
            _ => "some other pattern?".to_string()
        }
    }
}

impl ExplainParse for Ty_ {
    fn explain(&self) -> String {
        format!("{:?}", self)
    }
}

impl ExplainParse for Expr_ {
    fn explain(&self) -> String {
        format!("{:?}", self)
    }
}