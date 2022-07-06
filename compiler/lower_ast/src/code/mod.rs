mod func;

use blir::{
    code::{CodeBlock, Statement, StatementKind},
    typ::Type
};
use parser::ast::smt::{CodeBlock as AstCodeBlock, Smt as AstSmt};

use crate::{AstLowerer, err::Error};

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub(crate) fn lower_smt(&mut self, smt: AstSmt, last_loop_label: Option<&str>) -> Option<Statement> {
        let range = smt.range();
        let span = self.span(range);

        Some(match smt {
                 AstSmt::EvalSmt(smt) => {
                     let value = self.lower_expr(smt.value(), last_loop_label);
                     let escaped = smt.is_escaped();

                     StatementKind::Eval { value, escaped }
                 }

                 AstSmt::ReturnSmt(smt) => {
                     let value = smt.return_value()
                                    .map(|return_val| self.lower_expr(return_val, last_loop_label));

                     StatementKind::Return { value }
                 }

                 AstSmt::LetSmt(smt) => {
                     let pattern = self.lower_pattern(smt.pattern());
                     let typ = smt.typ()
                                  .map(|typ| self.lower_type(typ))
                                  .unwrap_or_else(Type::infer);
                     let value = smt.value().map(|expr| self.lower_expr(expr, last_loop_label));

                     StatementKind::Bind { pattern, typ, value }
                 }

                 AstSmt::BreakSmt(break_smt) => {
                    let label = break_smt.scope().or_else(|| last_loop_label.map(str::to_string));
                    // todo: break a value
                    if label.is_none() {
                        self.reporter.throw_diagnostic(Error::BreakOutsideLoop.at(span));
                    }
                    let value = break_smt.value().map(|val| self.lower_expr(val, last_loop_label));
                    StatementKind::Break(value, label.unwrap())
                 }

                 AstSmt::ContinueSmt(continue_smt) => {
                    let label = continue_smt.scope().or_else(|| last_loop_label.map(str::to_string));
                    if label.is_none() {
                        self.reporter.throw_diagnostic(Error::ContinueOutsideLoop.at(span));
                    }
                    StatementKind::Continue(label.unwrap())
                 },

                 AstSmt::GuardSmt(guard_smt) => {
                    if !feature_gate::has_feature("guard") {
                        self.reporter.throw_diagnostic(Error::FeatureNotEnabled("guard").at(span));
                    }

                    let condition = self.lower_expr(guard_smt.condition(), last_loop_label);

                    let code = self.lower_code_block(guard_smt.else_block(), last_loop_label);

                    StatementKind::Guard { condition: Box::new(condition), otherwise: code }
                 }

                 AstSmt::GuardLetSmt(guard_let_smt) => {
                    if !feature_gate::has_feature("guard_let") {
                        self.reporter.throw_diagnostic(Error::FeatureNotEnabled("guard_let").at(span));
                    }

                    let pattern = self.lower_pattern(guard_let_smt.pattern());
                    let value = self.lower_expr(guard_let_smt.value(), last_loop_label);
                    let code = self.lower_code_block(guard_let_smt.else_block(), last_loop_label);

                    StatementKind::GuardLet { pattern, value, otherwise: code }
                 }

                 AstSmt::NoOp(_) => return None,

                 AstSmt::Error => panic!(),
             }.spanned(span))
    }

    pub(crate) fn lower_code_block(&mut self, code_block: AstCodeBlock, last_loop_label: Option<&str>) -> CodeBlock {
        let range = code_block.range();
        let span = self.span(range);

        let statements = code_block.statements()
                                   .into_iter()
                                   .filter_map(|smt| self.lower_smt(smt, last_loop_label))
                                   .collect();

        CodeBlock::new(statements, span)
    }
}
