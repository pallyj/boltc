use blir::code::{CodeBlock, Statement, StatementKind};
use blirssa::value::LabelValue;

use crate::BlirLowerer;

impl<'a, 'b> BlirLowerer<'a, 'b> {
    pub(super) fn lower_code_block(&mut self, codeblock: &CodeBlock) -> Option<LabelValue> {
        let mut yield_value = None;

        for smt in codeblock.statements() {
            yield_value = self.lower_smt(smt);
        }

        yield_value
    }

    fn lower_smt(&mut self, smt: &Statement) -> Option<LabelValue> {
        match &smt.kind {
            StatementKind::Eval { value, escaped } => {
                let lowered_value = self.lower_value(value);

                if *escaped {
                    None
                } else {
                    Some(lowered_value)
                }
            }

            StatementKind::Return { value } => {
                let return_value = value.as_ref().map(|value| self.lower_value(value));

                self.builder().build_return(return_value);

                None
            }

            StatementKind::Bind { name, value, typ: _ } => {
                if let Some(value) = value {
                    let bind_value = self.lower_value(value);

                    self.context.define_var(name, bind_value);
                }

                None
            }
        }
    }
}
