use blir::{typ::{TypeKind, Type},
           value::{FunctionArgs, IfBranch, IfValue, Value, ValueKind, ClosureParam, Closure}};
use errors::Span;
use parser::ast::expr::{Expr as AstExpr, IfExpr, IfExprNegative, LiteralKind, ClosureExpr};

use crate::AstLowerer;

impl AstLowerer {
    pub(crate) fn lower_expr(&self, expr: AstExpr) -> Value {
        let range = expr.range();
        let span = self.span(range);

        match expr {
            AstExpr::NamedExpr(named) => ValueKind::Named(named.name()).spanned_infer(span),

            AstExpr::MemberExpr(member_expr) => ValueKind::Member { parent: Box::new(self.lower_expr(member_expr.parent())),
                                                                    member: member_expr.child().unwrap(), }.spanned_infer(span),

            AstExpr::PrefixExpr(prefix) => {
                let operator_symbol = prefix.operator();
                let operator = self.factory.get_prefix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function {
                    return_type: Box::new(Type::infer()),
                    params: vec![ Type::infer() ],
                    labels: vec![ None ] }.anon();

                ValueKind::FuncCall {
                    function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                    args: FunctionArgs {
                        args: vec![ self.lower_expr(prefix.unit()) ],
                        labels: vec![ None ] }}.spanned(Type::infer(), span)
            },
            AstExpr::PostfixExpr(postfix) => {
                let operator_symbol = postfix.operator();
                let operator = self.factory.get_postfix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function {
                    return_type: Box::new(Type::infer()),
                    params: vec![ Type::infer() ],
                    labels: vec![ None ] }.anon();

                ValueKind::FuncCall {
                    function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                    args: FunctionArgs {
                        args: vec![ self.lower_expr(postfix.unit()) ],
                        labels: vec![ None ] }}.spanned(Type::infer(), span)
            },
            AstExpr::InfixExpr(infix) => {
                let operator_symbol = infix.operator();
                let operator = self.factory.get_postfix_op(&operator_symbol).unwrap();

                let function = TypeKind::Function {
                    return_type: Box::new(Type::infer()),
                    params: vec![ Type::infer(), Type::infer() ],
                    labels: vec![ None, None ] }.anon();

                ValueKind::FuncCall {
                    function: Box::new(ValueKind::Operator(operator.name().clone()).anon(function)),
                    args: FunctionArgs {
                        args: vec![ self.lower_expr(infix.left()), self.lower_expr(infix.right()) ],
                        labels: vec![ None, None ] }}.spanned(Type::infer(), span)
            },

            AstExpr::LiteralExpr(literal) => {
                let text = literal.text().replace("_", "");

                match literal.literal_kind() {
                    LiteralKind::True => ValueKind::BoolLiteral(true).spanned_infer(span),
                    LiteralKind::False => ValueKind::BoolLiteral(false).spanned_infer(span),

                    LiteralKind::DecInteger => ValueKind::IntLiteral(str::parse(&text).unwrap()).spanned_infer(span),
                    LiteralKind::HexInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 16).unwrap()).spanned_infer(span),
                    LiteralKind::OctInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 8).unwrap()).spanned_infer(span),
                    LiteralKind::BinInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 2).unwrap()).spanned_infer(span),

                    LiteralKind::DecFloat => ValueKind::FloatLiteral(fast_float::parse(&text).unwrap()).spanned_infer(span),

                    _ => ValueKind::BoolLiteral(true).spanned_infer(span),
                }
            }

            AstExpr::ParenthesizedExpr(paren) => {
                // TODO: Add old span

                self.lower_expr(paren.expr())
            }

            AstExpr::FuncCallExpr(call) => {
                let mut func = self.lower_expr(call.function());

                let (labels, args): (Vec<_>, Vec<_>) =
                    call.args()
                        .map(|arg| (arg.label(), self.lower_expr(arg.value())))
                        .unzip();

                let return_type = Box::new( Type::infer() );
                let params = (0..args.len())
                    .map(|_| Type::infer() )
                    .collect();
                    
                let function_type = TypeKind::Function { return_type,
                                                         params,
                                                         labels: labels.clone() };
                func.typ.set_kind(function_type);

                ValueKind::FuncCall { function: Box::new(func),
                                      args:     FunctionArgs { args, labels }, }.spanned_infer(span)
            }

            AstExpr::ClosureExpr(closure) => self.lower_closure_expr(closure, span),

            AstExpr::TrailingClosureExpr(trailing_closure) => {
                let mut function = self.lower_expr( trailing_closure.function() );
                let closure = self.lower_closure_expr(trailing_closure.closure(), span);

                if let ValueKind::FuncCall { args, function: func } = &mut function.kind {
                    match func.typ.kind_mut() {
                        TypeKind::Function { params, .. } => {
                            params.push(Type::infer());
                        },
                        _ => panic!(),
                    }
                    args.args.push(closure);
                    function
                } else {
                    let return_type = Box::new( Type::infer() );
                    let params = vec![ Type::infer() ];
                        
                    let function_type = TypeKind::Function { return_type,
                                                            params,
                                                            labels: vec![] };
                    function.typ.set_kind(function_type);

                    ValueKind::FuncCall {
                        function: Box::new(function),
                        args: FunctionArgs {
                            args: vec![ closure ],
                            labels: vec![ ] }}.spanned_infer(span)
                }
            }

            AstExpr::IfExpr(expr) => {
                let if_value = self.lower_if_expr(expr);

                ValueKind::If(if_value).spanned_infer(span)
            }

            AstExpr::UnitExpr(_) => ValueKind::Unit.spanned(TypeKind::Void.anon(), span),

            AstExpr::Error => panic!(),
        }
    }

    fn lower_closure_expr(
        &self,
        closure: ClosureExpr,
        span: Span) -> Value
    {
        if let Some(parameters) = closure.parameters() {
            // Set the type to a function of the parameters
            let blir_params: Vec<_> = parameters
                .map(|closure_param| {
                    let typ = closure_param
                        .explicit_type()
                        .map(|typ| self.lower_type(typ))
                        .unwrap_or_else(|| Type::infer());

                    ClosureParam {
                        name: closure_param.bind_name(),
                        typ,
                    }
                }).collect();

            let function_parameter_types = blir_params
                .iter()
                .map(|param| param.typ.clone())
                .collect();

            let function_type = TypeKind::Function {
                return_type: Box::new(Type::infer()),
                params: function_parameter_types,
                labels: vec![] }.anon();

            let code = self.lower_code_block(closure.code_block());

            let closure = Closure {
                params: blir_params,
                code,
            };

            ValueKind::Closure(closure)
                .spanned(function_type, span)
        } else {
            let code = self.lower_code_block(closure.code_block());
            
            let closure = Closure {
                params: vec![],
                code
            };

            ValueKind::Closure(closure)
                .spanned_infer(span)
        }
    }

    pub(crate) fn lower_if_expr(&self, expr: IfExpr) -> IfValue {
        let condition = Box::new(self.lower_expr(expr.condition()));
        let positive = self.lower_code_block(expr.positive());
        let negative = match expr.negative() {
            Some(IfExprNegative::CodeBlock(cb)) => Some(IfBranch::CodeBlock(self.lower_code_block(cb))),
            Some(IfExprNegative::IfExpr(else_if)) => Some(IfBranch::Else(Box::new(self.lower_if_expr(else_if)))),
            _ => None,
        };

        IfValue { condition,
                  positive,
                  negative }
    }
}
