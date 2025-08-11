// Copyright 2025 Nord Security
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, Item, ItemId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_middle::ty::VariantDef;
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::{def_id::DefId, sym, Symbol};
use std::collections::HashSet;

// More info on this macro can be found:
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/macro.declare_lint.html
// https://rustc-dev-guide.rust-lang.org/diagnostics.html
declare_lint! {
    /// **What it does:** Checks for use of std::time::Instant and tokio::time::Instant
    ///
    /// **Why is this bad?** both types of instant don't take into the account the time spend sleeping / suspended. This can cause reacting to timeouts/timers with the delay equal to the sleep/suspend time.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // Bad
    /// use std::time::Instant;
    /// let start = Instant::now();
    /// // Code that might be interrupted by sleep/suspend
    /// let elapsed = start.elapsed();
    /// 
    /// // Also bad
    /// use tokio::time::Instant;
    /// let start = Instant::now();
    /// // Async code that might be interrupted by sleep/suspend
    /// let elapsed = start.elapsed();
    ///
    /// In libtelio there is already custom implementation of Instant that takes into account time spent in suspend.
    pub INSTANT,
    Warn,
    "default Instant implementations don't take into the account the time spent during suspend"
}

#[derive(Default)]
pub struct Instant{}

impl_lint_pass!(Instant => [INSTANT]);
impl LateLintPass<'_> for Instant {
    fn check_expr<'tcx>(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Call(fun, _args) = &expr.kind {
            if let ExprKind::Path(rustc_hir::QPath::TypeRelative(ty, path_segment)) = fun.kind {
                if path_segment.ident.as_str() == "now" {
                    // Get the type information
                    let ty = cx.typeck_results().node_type(ty.hir_id);
                    
                    let std_instant_paths = &[["std", "time", "Instant"].as_slice()];
                    let tokio_instant_paths = &[["tokio", "time", "instant", "Instant"].as_slice()];
                    
                    if let TyKind::Adt(adt_def, _) = ty.kind() {
                        let def_id = adt_def.did();
                        
                        if match_def_paths(cx, def_id, std_instant_paths).is_some() {
                            span_lint_and_help(
                                cx,
                                INSTANT,
                                expr.span,
                                "use of `std::time::Instant::now()` detected",
                                None,
                                "this Instant implementation doesn't account for system sleep/suspend time, which may lead to inaccurate timing measurements"
                            );
                        } else if match_def_paths(cx, def_id, tokio_instant_paths).is_some() {
                            span_lint_and_help(
                                cx,
                                INSTANT,
                                expr.span,
                                "use of `tokio::time::Instant::now()` detected",
                                None,
                                "this Instant implementation doesn't account for system sleep/suspend time, which may lead to inaccurate timing measurements"
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn match_def_paths<'a>(
    cx: &LateContext<'_>,
    def_id: DefId,
    syms: &'a [&'a [&str]],
) -> Option<&'a [&'a str]> {
    syms.iter()
        .find(|syms| match_def_path(cx, def_id, syms))
        .map(|syms| *syms)
}

// Replacement for clippy_utils::match_def_path which was removed in Rust 1.89
pub fn match_def_path(cx: &LateContext<'_>, def_id: DefId, path: &[&str]) -> bool {
    let def_path = cx.tcx.def_path(def_id);
    let crate_name = cx.tcx.crate_name(def_id.krate);
    
    // Build the full path starting with crate name
    let mut full_path = Vec::new();
    full_path.push(crate_name.to_string());
    
    for element in def_path.data.iter() {
        if let Some(name) = element.data.get_opt_name() {
            full_path.push(name.to_string());
        }
    }
    
    // Compare with the expected path
    if full_path.len() != path.len() {
        return false;
    }
    
    full_path.iter().zip(path.iter()).all(|(a, b)| a == b)
}
