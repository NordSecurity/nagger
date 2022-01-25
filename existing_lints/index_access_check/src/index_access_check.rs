// Copyright 2023 Nord Security
use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_ast::{Expr, ExprKind, Item, NodeId, RangeLimits};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::sym;

// More info on this macro can be found:
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/macro.declare_lint.html
// https://rustc-dev-guide.rust-lang.org/diagnostics.html
declare_lint! {
    /// **What it does:** Checks for the use of index [] access operator in code.
    ///
    /// **Why is this bad?** If the specified element at index does not exists - code panics.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// let map: HashMap<int32,int32> = HashMap::new();
    /// let member = map[2];
    /// ```
    /// Use instead:
    /// ```rust
    /// let map: HashMap<int32,int32> = HashMap::new();
    /// let member = map.get(2).ok_or(0)?;
    /// ```
    pub INDEX_ACCESS_CHECK,
    Warn,
    "index access operator panics, if out-of-bounds"
}

#[derive(Default)]
pub struct IndexAccessCheck {
    stack: Vec<NodeId>,
}

impl_lint_pass!(IndexAccessCheck => [INDEX_ACCESS_CHECK]);

impl EarlyLintPass for IndexAccessCheck {
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
        let mut warn_msg = "Use of Index [].";
        if_chain! {
            if !self.stack.is_empty();
            if let ExprKind::Index(_, ind_expr) = &expr.kind;
            then {
                if_chain!{
                    // Allow '[..]'
                    if let ExprKind::Range(None, None, RangeLimits::HalfOpen) = &ind_expr.kind;
                    then {} 
                    else {
                        if let ExprKind::Range(_, _, _) = &ind_expr.kind {
                            warn_msg = "Use of Range [x..y].";
                        }
                        span_lint_and_help(
                            cx,
                            INDEX_ACCESS_CHECK,
                            expr.span,
                            warn_msg,
                            None,
                            "Consider implementing proper member (or range) access or using #[allow(index_access_check)] to suppress the warning."
                        );
                    }
                }
            }
        }
    }
}

fn is_ignored(item: &Item) -> bool {
    item.attrs.iter().any(|attr| {
        if attr.has_name(sym::test) {
            // Ignores "#[test]"
            true
        } else {
            // Ignores "#[cfg(test)]"
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
