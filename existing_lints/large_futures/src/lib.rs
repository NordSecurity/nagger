// Copyright 2023 Nord Security
#![feature(rustc_private)]
#![allow(unused_imports)]

dylint_linting::dylint_library!();

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_parse_format;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;

mod large_futures;

const FUTURE_SIZE_THRESHOLD: u64 = 2048;

#[doc(hidden)]
#[no_mangle]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[large_futures::LARGE_FUTURES]);
    lint_store.register_late_pass(|_| Box::new(large_futures::LargeFutures::new(FUTURE_SIZE_THRESHOLD)));
}

// More info on tests https://github.com/trailofbits/dylint/tree/master/utils/testing
#[test]
fn ui() {
    dylint_testing::ui_test_example(env!("CARGO_PKG_NAME"), "ui");
}
