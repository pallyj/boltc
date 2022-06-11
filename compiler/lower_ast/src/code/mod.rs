mod func;

use blir::{code::{CodeBlock, Statement, StatementKind},
           typ::Type};
use errors::error::ErrorCode;
use parser::ast::smt::{CodeBlock as AstCodeBlock, Smt as AstSmt};

use crate::AstLowerer;

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
                     let name = smt.label();
                     let typ = smt.typ()
                                  .map(|typ| self.lower_type(typ))
                                  .unwrap_or_else(Type::infer);
                     let value = smt.value().map(|expr| self.lower_expr(expr, last_loop_label));

                     StatementKind::Bind { name, typ, value }
                 }

                 AstSmt::BreakSmt(_) => {
                     // todo: break a value and a loop
                     if last_loop_label.is_none() {
                        self.debugger.throw(ErrorCode::Other(String::from("can't use break statement outside a loop")), vec![span]);
                    }
                     let label = String::from(last_loop_label.unwrap_or(""));
                     StatementKind::Break(label)
                 }

                 AstSmt::ContinueSmt(_) => {
                     // todo: break a label
                     if last_loop_label.is_none() {
                         self.debugger.throw(ErrorCode::Other(String::from("can't use continue statement outside a loop")), vec![span]);
                     }
                     let label = String::from(last_loop_label.unwrap_or(""));
                     StatementKind::Continue(label)
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
