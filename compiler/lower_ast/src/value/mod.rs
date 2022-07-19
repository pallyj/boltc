mod unescape;

use std::{sync::atomic::{AtomicU64, Ordering}, vec};

use blir::{typ::{Type, TypeKind},
           value::{Closure, ClosureParam, FunctionArgs, IfBranch, IfValue, Value, ValueKind, match_::MatchValue, MatchBranch}, code::{CodeBlock, StatementKind}, pattern::PatternKind, attributes::{AttributeArg, AttributeArgs}};
use errors::Span;
use parser::{ast::expr::{ClosureExpr, Expr as AstExpr, IfExpr, IfExprNegative, LiteralKind, IfLetExpr, CollectionItem}};
use unindent::unindent;

use crate::{AstLowerer, err::Error};

use self::unescape::unescape;

static LOOP_COUNTER: AtomicU64 = AtomicU64::new(1);

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub(crate) fn lower_expr(&mut self, expr: AstExpr, last_loop_label: Option<&str>) -> Value {
        let range = expr.range();
        let span = self.span(range);

        match expr {
            AstExpr::NamedExpr(named) => ValueKind::Named(named.name()).spanned_infer(span),

            AstExpr::MemberExpr(member_expr) => ValueKind::Member { parent: Box::new(self.lower_expr(member_expr.parent(), last_loop_label)),
                                                                    member: member_expr.child().unwrap(), }.spanned_infer(span),

            AstExpr::TupleExpr(tuple_expr) => {
                let (tuple_items, labels): (Vec<_>, Vec<_>) =
                    tuple_expr.items()
                              .map(|item| (self.lower_expr(item.expr(), last_loop_label), item.label()))
                              .unzip();

                let infer_items = (0..tuple_items.len()).map(|_| Type::infer()).collect();
                let tuple_type = TypeKind::Tuple(infer_items, labels).anon();

                ValueKind::Tuple(tuple_items).spanned(tuple_type, span)
            }

            AstExpr::PrefixExpr(prefix) => {
                let operator_symbol = prefix.operator();
                let operator = self.factory.get_prefix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                    params:      vec![Type::infer()],
                                                    labels:      vec![None], }.anon();

                ValueKind::FuncCall {
                    function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                    args:     FunctionArgs {
                        args:   vec![self.lower_expr(prefix.unit(), last_loop_label)],
                        labels: vec![None],
                        is_shared: vec![false] }, }.spanned(Type::infer(), span)
            }
            AstExpr::PostfixExpr(postfix) => {
                let operator_symbol = postfix.operator();
                let operator = self.factory.get_postfix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                    params:      vec![Type::infer()],
                                                    labels:      vec![None], }.anon();

                ValueKind::FuncCall {
                    function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                    args:     FunctionArgs {
                        args:   vec![self.lower_expr(postfix.unit(), last_loop_label)],
                        labels: vec![None],
                        is_shared: vec![false]
                    },
                }.spanned(Type::infer(), span)
            }
            AstExpr::InfixExpr(infix) => {
                let operator_symbol = infix.operator();

                if operator_symbol == "=" {
                    let left = self.lower_expr(infix.left(), last_loop_label);
                    let right = self.lower_expr(infix.right(), last_loop_label);

                    return ValueKind::Assign(Box::new(left), Box::new(right))
                                    .spanned(TypeKind::Void.spanned(span), span)
                }
                if !["!=", "==", "<=", ">="].contains(&operator_symbol.as_str()) {
                    if let Some(operator_symbol) = operator_symbol.strip_suffix('=') {
                        let operator = self.factory.get_postfix_op(operator_symbol).unwrap();

                        let function = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                            params:      vec![Type::infer(), Type::infer()],
                                                            labels:      vec![None, None], }.anon();

                        let left = self.lower_expr(infix.left(), last_loop_label);
                        let right = self.lower_expr(infix.right(), last_loop_label);
        
                        let assign_val = ValueKind::FuncCall {
                            function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                            args:     FunctionArgs {
                                args:   vec![left.clone(), right],
                                labels: vec![None, None],
                                is_shared: vec![false, false]
                        } }.spanned(Type::infer(), span);

                        return ValueKind::Assign(Box::new(left), Box::new(assign_val))
                                        .spanned(TypeKind::Void.spanned(span), span)                                   
                    }
                }
                let operator = self.factory.get_postfix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                    params:      vec![Type::infer(), Type::infer()],
                                                    labels:      vec![None, None], }.anon();

                ValueKind::FuncCall { function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                                        args:     FunctionArgs { args:   vec![self.lower_expr(infix.left(), last_loop_label),
                                                                            self.lower_expr(infix.right(), last_loop_label)],
                                                                labels: vec![None, None],
                                                                is_shared: vec![false, false] }, }.spanned(Type::infer(), span)

                
            }
            AstExpr::IndexExpr(index) => {
                let operator = "index".to_string();

                let index_arg = index.index();
                let label = index_arg.label();
                let value = index_arg.value();

                let function = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                    params:      vec![Type::infer(), Type::infer()],
                                                    labels:      vec![None, label.clone()], }.anon();

                ValueKind::FuncCall { function: Box::new(ValueKind::Operator(operator).anon(function)),
                                      args:     FunctionArgs { args:   vec![self.lower_expr(index.parent(), last_loop_label),
                                                                            self.lower_expr(value, last_loop_label)],
                                                               labels: vec![None, label],
                                                               is_shared: vec![false, false] }, }.spanned(Type::infer(), span)
            }

            AstExpr::LiteralExpr(literal) => {
                let kind = literal.literal_kind();

                if let LiteralKind::String = kind {
                    let text = literal.text();
                    let text_range = 1..(text.len() - 1);

                    match unescape(&text[text_range]) {
                        Ok(unescaped) => ValueKind::StringLiteral(unescaped).spanned_infer(span),
                        Err(errs) => {
                            for err in errs {
                                let start =  usize::from(span.range().start());
                                let range = (start + err.1, start + err.1 + 1);
                                let span = (self.file as usize, range);

                                self.reporter.throw_diagnostic(Error::Unicode(err.0).at_raw(span));
                            }

                            ValueKind::Error.spanned_infer(span)
                        }
                    }
                } else if let LiteralKind::LongString = kind {
                    let text = literal.text();
                    let text_range = 3..(text.len() - 3);
                    let unindent = unindent(&text[text_range]);

                    match unescape(&unindent) {
                        Ok(unescaped) => ValueKind::StringLiteral(unescaped).spanned_infer(span),
                        Err(errs) => {
                            for err in errs {
                                let start =  usize::from(span.range().start());
                                let range = (start + err.1, start + err.1 + 1);
                                let span = (self.file as usize, range);

                                self.reporter.throw_diagnostic(Error::Unicode(err.0).at_raw(span));
                            }

                            ValueKind::Error.spanned_infer(span)
                        }
                    }
                } else {
                    let text = literal.text().replace("_", "");

                    match literal.literal_kind() {
                        LiteralKind::True => ValueKind::BoolLiteral(true).spanned_infer(span),
                        LiteralKind::False => ValueKind::BoolLiteral(false).spanned_infer(span),

                        LiteralKind::DecInteger => ValueKind::IntLiteral(str::parse(&text).unwrap()).spanned_infer(span),
                        LiteralKind::HexInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 16).unwrap()).spanned_infer(span),
                        LiteralKind::OctInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 8).unwrap()).spanned_infer(span),
                        LiteralKind::BinInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 2).unwrap()).spanned_infer(span),

                        LiteralKind::DecFloat => ValueKind::FloatLiteral(fast_float::parse(&text).unwrap()).spanned_infer(span),

                        LiteralKind::String => unreachable!(),
                        LiteralKind::LongString => unreachable!(),
                        LiteralKind::Error => panic!("internal compiler error"),
                    }
                }
            }

            AstExpr::ParenthesizedExpr(paren) => {
                // TODO: Add old span
                let expr = self.lower_expr(paren.expr(), last_loop_label);

                if let Some(label) = paren.tuple_label() {
                    let (tuple_items, labels) = (vec![expr], vec![Some(label)]);

                    let infer_items = vec! [Type::infer()];
                    let tuple_type = TypeKind::Tuple(infer_items, labels).anon();

                    ValueKind::Tuple(tuple_items).spanned(tuple_type, span)
                } else {
                    expr
                }
            }

            AstExpr::FuncCallExpr(call) => {
                let mut func = self.lower_expr(call.function(), last_loop_label);

                let (labels, args): (Vec<_>, Vec<_>) = call.args()
                                                           .map(|arg| (arg.label(), self.lower_expr(arg.value(), last_loop_label)))
                                                           .unzip();

                let is_shared = call.args().map(|arg| arg.is_shared()).collect();

                let return_type = Box::new(Type::infer());
                let params = (0..args.len()).map(|_| Type::infer()).collect();

                let function_type = TypeKind::Function { return_type,
                                                         params,
                                                         labels: labels.clone() };
                func.typ.set_kind(function_type);

                ValueKind::FuncCall { function: Box::new(func),
                                      args:     FunctionArgs { args, labels, is_shared }, }.spanned_infer(span)
            }

            AstExpr::ClosureExpr(closure) => self.lower_closure_expr(closure, span),

            AstExpr::TrailingClosureExpr(trailing_closure) => {
                let mut function = self.lower_expr(trailing_closure.function(), last_loop_label);
                let closure = self.lower_closure_expr(trailing_closure.closure(), span);

                if let ValueKind::FuncCall { args, function: func } = &mut function.kind {
                    match func.typ.kind_mut() {
                        TypeKind::Function { params, .. } => {
                            params.push(Type::infer());
                        }
                        _ => self.reporter.throw_diagnostic(Error::NoTrailingClosure.at(span)),
                    }
                    args.args.push(closure);
                    args.is_shared.push(false);
                    function
                } else {
                    let return_type = Box::new(Type::infer());
                    let params = vec![Type::infer()];

                    let function_type = TypeKind::Function { return_type,
                                                             params,
                                                             labels: vec![] };
                    function.typ.set_kind(function_type);

                    ValueKind::FuncCall { function: Box::new(function),
                                          args:     FunctionArgs { args:   vec![closure],
                                                                   labels: vec![],
                                                                   is_shared: vec![ false ] }, }.spanned_infer(span)
                }
            }

            AstExpr::IfExpr(expr) => {
                let if_value = self.lower_if_expr(expr, last_loop_label);

                ValueKind::If(if_value).spanned_infer(span)
            }

            AstExpr::UnitExpr(_) => ValueKind::Unit.spanned(TypeKind::Void.anon(), span),

            AstExpr::VariantLiteral(variant_expr) => ValueKind::VariantLiteral(variant_expr.variant_name()).spanned(Type::infer(), span),

            AstExpr::MatchExpr(match_expr) => {
                let discriminant = Box::new(self.lower_expr(match_expr.discriminant(), last_loop_label));

                let branches = match_expr.branches()
                    .map(|branch| {
                        let pattern = self.lower_pattern(branch.pattern());

                        let code = if let Some(code_block) = branch.code_block() {
                            self.lower_code_block(code_block, last_loop_label)
                        } else {
                            let value = self.lower_expr(branch.value().unwrap(), last_loop_label);
                            let span = value.span.unwrap();
                            let statement = StatementKind::Eval { value, escaped: false }.spanned(span);

                            CodeBlock::new(vec! [statement], span)
                        };

                        MatchBranch { pattern, code }
                    })
                    .collect();

                ValueKind::Match(MatchValue { discriminant, branches }).spanned_infer(span)
            }

            AstExpr::IfLetExpr(if_let_expr) => ValueKind::Match(self.lower_if_let_expr(if_let_expr, last_loop_label)).spanned_infer(span),

            AstExpr::RepeatLoop(repeat_loop) => {
                if !feature_gate::has_feature("repeat_loops") {
                    self.reporter.throw_diagnostic(Error::FeatureNotEnabled("repeat_loops").at(span));
                }

                let next_label = repeat_loop.scope().unwrap_or_else(|| format!("repeat#{}", LOOP_COUNTER.fetch_add(1, Ordering::Relaxed)));
                let lowered_block = self.lower_code_block(repeat_loop.code_block(), Some(&next_label));

                ValueKind::Loop{ code: lowered_block, label: next_label }.spanned_infer(span)
            }

            AstExpr::WhileLoop(while_loop) => {
                if !feature_gate::has_feature("while_loops") {
                    self.reporter.throw_diagnostic(Error::FeatureNotEnabled("while_loops").at(span));
                }

                let condition = self.lower_expr(while_loop.condition(), last_loop_label);

                let next_label = while_loop.scope().unwrap_or_else(|| format!("while#{}", LOOP_COUNTER.fetch_add(1, Ordering::Relaxed)));
                let lowered_block = self.lower_code_block(while_loop.code_block(), Some(&next_label));
                let else_break = CodeBlock::new(vec![ StatementKind::Break(None, next_label.clone()).spanned(span) ], span);

                let if_switch = ValueKind::If(IfValue {
                    condition: Box::new(condition),
                    positive: lowered_block,
                    negative: Some(IfBranch::CodeBlock(else_break)),
                }).infer();

                let if_block = CodeBlock::new(vec![
                    StatementKind::Eval { value: if_switch, escaped: false }.spanned(span)
                ], span);



                ValueKind::Loop{ code: if_block, label: next_label }.spanned_infer(span)
            }

            AstExpr::WhileLetLoop(while_let_loop) => {
                if !feature_gate::has_feature("while_let_loops") {
                    self.reporter.throw_diagnostic(Error::FeatureNotEnabled("while_let_loops").at(span));
                }

                let scrutinee = self.lower_expr(while_let_loop.value(), last_loop_label);
                let pattern = self.lower_pattern(while_let_loop.pattern());

                let next_label = while_let_loop.scope().unwrap_or_else(|| format!("while#{}", LOOP_COUNTER.fetch_add(1, Ordering::Relaxed)));
                let lowered_block = self.lower_code_block(while_let_loop.code_block(), Some(&next_label));
                let else_break = CodeBlock::new(vec![ StatementKind::Break(None, next_label.clone()).spanned(span) ], span);

                let match_pat = ValueKind::Match(MatchValue {
                    discriminant: Box::new(scrutinee),
                    branches: vec![
                        MatchBranch { pattern, code: lowered_block },
                        MatchBranch { pattern: PatternKind::Wildcard.with_span(span), code: else_break }
                    ]
                }).infer();

                let if_block = CodeBlock::new(vec![
                    StatementKind::Eval { value: match_pat, escaped: false }.spanned(span)
                ], span);



                ValueKind::Loop{ code: if_block, label: next_label }.spanned_infer(span)
            }

            AstExpr::ArrayLiteral(array_literal) => {
                let items = array_literal.items();

                let is_sequence = items.iter().all(|item| matches!(item, CollectionItem::ArrayItem(_)));

                if is_sequence {
                    let sequence_items = items.iter()
                        .map(|item| {
                            match item {
                                CollectionItem::ArrayItem(array_item) => self.lower_expr(array_item.item(), None),
                                _ => unreachable!()
                            }
                        })
                        .collect::<Vec<_>>();

                    let ty = TypeKind::Array {
                        item: Box::new(Type::infer()),
                        len: sequence_items.len()
                    }.spanned(span);

                    return ValueKind::SequenceLiteral(sequence_items).spanned(ty, span)
                }

                if items.len() == 1 {
                    if let CollectionItem::MapItem(map_item) = &items[0] {
                        let key = map_item.key();

                        if let AstExpr::NamedExpr(named_expr) = key &&
                           named_expr.name() == "repeating" {

                            return ValueKind::RepeatingLiteral {
                                repeating: Box::new(self.lower_expr(map_item.value(), None)),
                                count: None
                            }.spanned_infer(span)
                        }
                    }
                } else if items.len() == 2 {
                    if let CollectionItem::MapItem(repeating) = &items[0] &&
                       let CollectionItem::MapItem(count) = &items[1] {

                        if let AstExpr::NamedExpr(repeating_named) = repeating.key() &&
                           let AstExpr::NamedExpr(count_named) = count.key() &&
                           repeating_named.name() == "repeating" &&
                           count_named.name() == "count" {

                            let value = self.lower_expr(count.value(), None);
                            let (is_negative, count) = self.lower_integer(&value);

                            if is_negative {
                                self.reporter.throw_diagnostic(Error::NegativeArrayLength(count).at(value.span.unwrap_or_default()));
                            }

                            let typ = TypeKind::Array { item: Box::new(Type::infer()), len: count as usize }.spanned(span);

                            return ValueKind::RepeatingLiteral {
                                repeating: Box::new(self.lower_expr(repeating.value(), None)),
                                count: Some(count)
                            }.spanned(typ, span)
                        }

                    }
                }

                // Throw an error
                self.reporter.throw_diagnostic(Error::FeatureNotEnabled("map_lit_construct").at(span));

                ValueKind::Error.infer()
            }

            AstExpr::Macro(macro_def) => {
                let macro_name = macro_def.macro_name();

               let args = macro_def.args()
                    .map(|args|
                    args.filter_map(|arg| {
                        let label = arg.label();
                        let value = self.lower_expr(arg.value(), None);
                        let arg_val = match &value.kind {
                            blir::value::ValueKind::Named(name) => AttributeArg::Named(name.clone()),
                            blir::value::ValueKind::IntLiteral(n) => AttributeArg::Integer(*n),
                            blir::value::ValueKind::FloatLiteral(n) => AttributeArg::Float(*n),
                            blir::value::ValueKind::BoolLiteral(b) => AttributeArg::Bool(*b),
                            blir::value::ValueKind::StringLiteral(s) => AttributeArg::String(s.clone()),
                            blir::value::ValueKind::VariantLiteral(v) => AttributeArg::Variant(v.clone()),
                            _ => { return None }
                        };

                        Some((label, arg_val))
                    })
                    .collect()).unwrap_or_else(|| Vec::new());

                let attribute_args = AttributeArgs::new(args);

                ValueKind::Macro(macro_name, attribute_args).spanned_infer(span)
            }

            AstExpr::Error => panic!("internal compiler error")
        }
    }

    fn lower_closure_expr(&mut self, closure: ClosureExpr, span: Span) -> Value {
        if let Some(parameters) = closure.parameters() {
            // Set the type to a function of the parameters
            let blir_params: Vec<_> = parameters.map(|closure_param| {
                                                    let typ = closure_param.explicit_type()
                                                                           .map(|typ| self.lower_type(typ))
                                                                           .unwrap_or_else(Type::infer);

                                                    ClosureParam { name: closure_param.bind_name(),
                                                                   typ }
                                                })
                                                .collect();

            let function_parameter_types = blir_params.iter().map(|param| param.typ.clone()).collect();

            let function_type = TypeKind::Function { return_type: Box::new(Type::infer()),
                                                     params:      function_parameter_types,
                                                     labels:      vec![], }.anon();

            let code = self.lower_code_block(closure.code_block(), None);

            let closure = Closure { params: blir_params,
                                    code };

            ValueKind::Closure(closure).spanned(function_type, span)
        } else {
            let code = self.lower_code_block(closure.code_block(), None);

            let closure = Closure { params: vec![], code };

            let closure_type = Type::infer();

            ValueKind::Closure(closure).spanned(closure_type, span)
        }
    }

    pub(crate) fn lower_if_expr(&mut self, expr: IfExpr, last_loop_label: Option<&str>) -> IfValue {
        let condition = Box::new(self.lower_expr(expr.condition(), last_loop_label));
        let positive = self.lower_code_block(expr.positive(), last_loop_label);
        let negative = match expr.negative() {
            Some(IfExprNegative::CodeBlock(cb)) => Some(IfBranch::CodeBlock(self.lower_code_block(cb, last_loop_label))),
            Some(IfExprNegative::IfExpr(else_if)) => Some(IfBranch::Else(Box::new(self.lower_if_expr(else_if, last_loop_label)))),
            Some(IfExprNegative::IfLetExpr(else_if)) => Some(IfBranch::ElseLet(Box::new(self.lower_if_let_expr(else_if, last_loop_label)))),
            _ => None,
        };

        IfValue { condition,
                  positive,
                  negative }
    }

    pub(crate) fn lower_if_let_expr(&mut self, expr: IfLetExpr, last_loop_label: Option<&str>) -> MatchValue {
        let span = self.span(expr.range());

        if !feature_gate::has_feature("if_let") {
            self.reporter.throw_diagnostic(Error::FeatureNotEnabled("if_let").at(span));
        }

        let neg_span = expr.negative().map(|expr| self.span(expr.range()));

        let scrutinee = self.lower_expr(expr.value(), last_loop_label);
        let pattern = self.lower_pattern(expr.pattern());

        let next_label = format!("while#{}", LOOP_COUNTER.fetch_add(1, Ordering::Relaxed));
        let lowered_block = self.lower_code_block(expr.positive(), Some(&next_label));
        let else_break = match expr.negative() {
            Some(IfExprNegative::CodeBlock(cb)) => self.lower_code_block(cb, last_loop_label),
            Some(IfExprNegative::IfExpr(else_if)) => CodeBlock::new(vec![
                StatementKind::Eval { value: ValueKind::If(self.lower_if_expr(else_if, last_loop_label)).spanned_infer(neg_span.unwrap()) , escaped: false }.spanned(neg_span.unwrap())
            ], neg_span.unwrap()),
            Some(IfExprNegative::IfLetExpr(else_if)) => CodeBlock::new(vec![
                StatementKind::Eval { value: ValueKind::Match(self.lower_if_let_expr(else_if, last_loop_label)).spanned_infer(neg_span.unwrap()) , escaped: false }.spanned(neg_span.unwrap())
            ], neg_span.unwrap()),
            None => CodeBlock::new(vec![], span),
            Some(IfExprNegative::Error) => unreachable!(),
        };

        MatchValue {
            discriminant: Box::new(scrutinee),
            branches: vec![
                MatchBranch { pattern, code: lowered_block },
                MatchBranch { pattern: PatternKind::Wildcard.with_span(span), code: else_break }
            ]
        }
    }
}
