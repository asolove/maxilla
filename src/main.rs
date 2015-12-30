#![feature(rustc_private)]
extern crate syntax;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_plugin;

use rustc::session::config;
use rustc::session::config::Input;
use rustc_driver::driver;

fn main() {
    println!("Hello, world!");
}

fn parse_code(code: &str) {
    let config = panic!("Kaboom");
    let sopts = config::basic_options();
    let registry = rustc_plugin::registry::Registry::new(rustc::DIAGNOSTICS);

    let sess = build_session(sopts, Some("input.rs"), registry);
    let cfg = build_configuration(&sess);
    let input = Input::Str("fn main() { println!(\"Hello, world!\"); }".to_string());
    let krate = driver::phase_1_parse_input(&sess, cfg, &input);
}

