mod func;

use blir::{code::{CodeBlock, Statement, StatementKind},
           typ::Type};
use parser::ast::smt::{CodeBlock as AstCodeBlock, Smt as AstSmt};

use crate::AstLowerer;

impl AstLowerer {
    pub(crate) fn lower_smt(&self, smt: AstSmt) -> Option<Statement> {
        let range = smt.range();
        let span = self.span(range);

        Some(match smt {
                 AstSmt::EvalSmt(smt) => {
                     let value = self.lower_expr(smt.value());
                     let escaped = smt.is_escaped();

                     StatementKind::Eval { value, escaped }
                 }

                 AstSmt::ReturnSmt(smt) => {
                     let value = smt.return_value()
                                    .map(|return_val| self.lower_expr(return_val));

                     StatementKind::Return { value }
                 }

                 AstSmt::LetSmt(smt) => {
                     let name = smt.label();
                     let typ = smt.typ()
                                  .map(|typ| self.lower_type(typ))
                                  .unwrap_or_else(|| Type::infer());
                     let value = smt.value().map(|expr| self.lower_expr(expr));

                     StatementKind::Bind { name, typ, value }
                 }

                 AstSmt::NoOp(_) => return None,

                 AstSmt::Error => panic!(),
             }.spanned(span))
    }

    pub(crate) fn lower_code_block(&self, code_block: AstCodeBlock) -> CodeBlock {
        let range = code_block.range();
        let span = self.span(range);

        let statements = code_block.statements()
                                   .into_iter()
                                   .filter_map(|smt| self.lower_smt(smt))
                                   .collect();

        CodeBlock::new(statements, span)
    }
}
