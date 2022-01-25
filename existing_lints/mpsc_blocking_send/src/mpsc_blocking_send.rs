// Copyright 2023 Nord Security
use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::{Expr, ExprKind, Item, ItemId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_middle::ty::VariantDef;
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::sym;
use std::collections::HashSet;

// More info on this macro can be found:
// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/macro.declare_lint.html
// https://rustc-dev-guide.rust-lang.org/diagnostics.html
declare_lint! {
    /// **What it does:** Checks for use of blocking mpsc::[Sync]Sender::send() calls
    ///
    /// **Why is this bad?** This might deadlock if used incorrectly
    ///
    /// **Example:**
    ///
    /// If the other end is not ready or too slow to receive, sending might overflow allowed receive buffer and
    /// block or even create deadlock, best example is in use with select! macros.
    ///
    /// We should use try_send() or get permit, to make sure the other end is ready to receive data
    pub MPSC_BLOCKING_SEND,
    Warn,
    "description goes here"
}

#[derive(Default)]
pub struct MpscBlockingSend {}

impl_lint_pass!(MpscBlockingSend => [MPSC_BLOCKING_SEND]);

impl LateLintPass<'_> for MpscBlockingSend {
    fn check_expr<'tcx>(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if_chain! {
            if let ExprKind::MethodCall(func, func_expr, params, _) = &expr.kind;
            if func.ident.name.as_str() == "send";
            if params.len() >= 1;
            if let TyKind::Adt(def, _) = cx.typeck_results().node_type(func_expr.hir_id).kind();
            if def.is_struct();
            then {
                let type_def = format!("{:?}", def);
                if type_def == "tokio::sync::mpsc::Sender" || type_def == "std::sync::mpsc::SyncSender" {
                    span_lint_and_help(
                        cx,
                        MPSC_BLOCKING_SEND,
                        expr.span,
                        "Use of blocking mpsc::[Sync]Sender::send() variant, this might deadlock if used incorrectly",
                        None,
                        "Consider using try_send() or wait_for_tx(). Alternatively use #[allow(mpsc_blocking_send)] to suppress the warning."
                    );
                }
            }
        }
    }
}
