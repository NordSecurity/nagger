// Copyright 2023 Nord Security
use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_ast::{Expr, ExprKind, Item, NodeId};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::sym;

// More info on this macro can be found:
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/macro.declare_lint.html
// https://rustc-dev-guide.rust-lang.org/diagnostics.html
declare_lint! {
    /// **What it does:** Checks for the use of unwrap() in code.
    ///
    /// **Why is this bad?** It's generally a bad idea to use unwrap and leave a potential error unhandled.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// std::process::Command::new("env").status().unwrap();
    /// ```
    /// Use instead:
    /// ```rust
    /// std::process::Command::new("env").status()?;
    /// ```
    pub UNWRAP_CHECK,
    Warn,
    "description goes here"
}

#[derive(Default)]
pub struct UnwrapCheck {
    stack: Vec<NodeId>,
}

impl_lint_pass!(UnwrapCheck => [UNWRAP_CHECK]);

impl EarlyLintPass for UnwrapCheck {
    fn check_item(&mut self, _cx: &EarlyContext, item: &Item) {
        if !self.stack.is_empty() || !is_ignored(item) {
            self.stack.push(item.id);
        }
    }

    fn check_item_post(&mut self, _cx: &EarlyContext, item: &Item) {
        if let Some(node_id) = self.stack.pop() {
            assert_eq!(node_id, item.id);
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.stack.is_empty();
            if let ExprKind::MethodCall(func) = &expr.kind;
            if func.seg.ident.name.as_str() == "unwrap";
            then {
                span_lint_and_help(
                    cx,
                    UNWRAP_CHECK,
                    expr.span,
                    "Use of unwrap().",
                    None,
                    "Consider implementing proper error handling or using #[allow(unwrap_check)] to suppress the warning."
                );
            }
        }
    }
}

fn is_ignored(item: &Item) -> bool {
    item.attrs.iter().any(|attr| {
        if attr.has_name(sym::test) {
            true
        } else {
            if_chain! {
                if attr.has_name(sym::cfg);
                if let Some(items) = attr.meta_item_list();
                if let [item] = items.as_slice();
                if let Some(feature_item) = item.meta_item();
                if feature_item.has_name(sym::test);
                then {
                    true
                } else {
                    false
                }
            }
        }
    })
}
