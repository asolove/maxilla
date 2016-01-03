// Parsing/explaining requirements, should be own module
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

// Server requirements
extern crate iron;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|req: &mut Request| {
      let mut code = "fn main() { let greeting = greet(\"Dave\"); print!(greeing); }\nfn greet(name: &str) -> &str { \"Hello, \" + name }".to_string();
      let body = req.get::<bodyparser::Raw>();
      match body {
          Ok(Some(body)) => code = body,
          // FIXME: error handling if nothing or invalid thing provided
          Ok(None) => println!("No body"),
          Err(err) => println!("Error: {:?}", err)
      }
      let krate = parse_code(&code);
      let explanation = krate.module.spanned_explain();
      // FIXME: parser currently just panics with invalid code. Catch and return a meaningful error.
      let response = tree::annotate(explanation, &code, 0);
        Ok(Response::with((status::Ok, response)))
    }).http("localhost:3000").unwrap();
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