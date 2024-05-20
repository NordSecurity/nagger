// Copyright 2024 Nord Security
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, Item, ItemId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_middle::ty::VariantDef;
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::{def_id::DefId, sym};
use std::collections::HashSet;

// More info on this macro can be found:
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/macro.declare_lint.html
// https://rustc-dev-guide.rust-lang.org/diagnostics.html
declare_lint! {
    /// **What it does:** Checks for use of blocking tokio::time::interval(_at) calls
    ///
    /// **Why is this bad?** This might cause generation of many events if used incorrectly
    ///
    /// **Example:**
    ///
    /// If there is some periodic action that is triggered by the resulting Interval instance,
    /// after a long period of sleep it will be triggered 'sleep_duration/interval_period' as fast
    /// as possible until it is caught up in time to where it should be. Most of the time this is
    /// not a desired behaviour. This default behaviour can be controlled with the 'set_missed_tick_behavior'
    /// setter. In libtelio there is already a wrapper that returns correctly configured interval.
    pub TOKIO_TIME_INTERVAL,
    Warn,
    "tokio::time::interval (with the default missed tick behaviour) generates lots of ticks after a sleep"
}

#[derive(Default)]
pub struct TokioTimeInterval {}

impl_lint_pass!(TokioTimeInterval => [TOKIO_TIME_INTERVAL]);

impl LateLintPass<'_> for TokioTimeInterval {
    fn check_expr<'tcx>(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Call(fun, _args) = &expr.kind {
            if let ExprKind::Path(rustc_hir::QPath::Resolved(_, path)) = fun.kind {
                if let Some(path) = path.res.opt_def_id() {
                    if let Some(syms) = match_def_paths(
                        cx,
                        path,
                        &[
                            &["tokio", "time", "interval", "interval"],
                            &["tokio", "time", "interval", "interval_at"],
                        ],
                    ) {
                        let name = syms.join("::");
                        span_lint_and_help(
                            cx,
                            TOKIO_TIME_INTERVAL,
                            expr.span,
                            &format!("Use of {name}, this might generate many ticks after a sleep."),
                            None,
                            "Consider using wrapper from telio-utils, or set the missed tick behaviour to appropriate value."
                        );
                    }
                }
            }
        }
    }
}

pub fn match_def_paths<'a>(
    cx: &LateContext<'_>,
    path: DefId,
    syms: &'a [&'a [&str]],
) -> Option<&'a [&'a str]> {
    syms.iter()
        .find(|syms| clippy_utils::match_def_path(cx, path, syms))
        .map(|syms| *syms)
}
