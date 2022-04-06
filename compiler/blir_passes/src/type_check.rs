use blir::{code::{CodeBlock, FunctionRef, MethodRef, Statement, StatementKind},
           typ::{StructRef, Type, TypeKind},
           value::{IfBranch, IfValue, Value, ValueKind},
           Library};
use errors::{debugger::Debugger, error::ErrorCode, Span};

pub fn run_pass(library: &mut Library, debugger: &mut Debugger) {
    for func in library.functions.iter() {
        check_function(func, debugger);
    }

    for r#struct in library.structs.iter() {
        check_struct(r#struct, debugger);
    }
}

fn check_function(func: &FunctionRef, debugger: &mut Debugger) {
    let func = func.borrow();
    check_codeblock(&func.code, func.info.return_type(), debugger);
}

fn check_struct(r#struct: &StructRef, debugger: &mut Debugger) {
    let r#struct = r#struct.borrow();
    for func in r#struct.methods.iter() {
        check_method(func, debugger);
    }
}

fn check_method(func: &MethodRef, debugger: &mut Debugger) {
    let func = func.borrow();
    check_codeblock(&func.code, func.info.return_type(), debugger);
}

fn check_codeblock(code_block: &CodeBlock, return_type: &Type, debugger: &mut Debugger) {
    for smt in code_block.statements() {
        check_smt(smt, return_type, debugger);
    }

    let ret = code_block.typ();

    if !is_assignable_from(return_type, &ret) {
        let expected = format!("{return_type:?}");
        let found = format!("{ret:?}");

        debugger.throw_single(ErrorCode::ExpectedFound(expected, found), &ret.span);
    }
}

fn check_smt(smt: &Statement, return_type: &Type, debugger: &mut Debugger) {
    match &smt.kind {
        StatementKind::Eval { value, escaped: _ } => {
            check_value(value, return_type, debugger);
        }
        StatementKind::Bind { name: _, typ, value } => {
            if let Some(value) = value.as_ref() {
                check_value(&value, return_type, debugger);

                if !is_assignable_from(typ, &value.typ) {
                    let expected_ty = format!("{:?}", typ);
                    let found = format!("{:?}", value.typ);

                    debugger.throw_single(ErrorCode::ExpectedFound(expected_ty, found), &value.span);
                }
            }
        }
        StatementKind::Return { value } => {
            if let Some(value) = value.as_ref() {
                check_value(&value, return_type, debugger);

                if !is_assignable_from(return_type, &value.typ) {
                    let expected_ty = format!("{:?}", return_type);
                    let found = format!("{:?}", value.typ);

                    debugger.throw_single(ErrorCode::ExpectedFound(expected_ty, found), &value.span);
                }
            }
        }
    }
}

fn check_value(value: &Value, return_type: &Type, debugger: &mut Debugger) {
    // if &value.typ != return_type {
    // println!("Error Values");
    // }

    match &value.kind {
        ValueKind::FuncCall { function, args } => {
            // Match function args

            check_value(function.as_ref(), return_type, debugger);

            let params = match function.typ.kind() {
                TypeKind::Function { params, .. } => params,
                TypeKind::Method { params, .. } => params,
                _ => {
                    debugger.throw_single(ErrorCode::IsNotAFunc, &function.span);
                    return;
                }
            };

            if args.args.len() > params.len() {
                // Extra args
                let extra_spans = args.args[params.len()..args.args.len()].iter()
                                                                          .filter_map(|arg| arg.span.clone())
                                                                          .collect();

                debugger.throw(ErrorCode::ExtraParams, extra_spans);
            } else if args.args.len() < params.len() {
                // Missing args
                debugger.throw_single(ErrorCode::MissingParams, &value.span);
            }

            for (arg, param) in args.args.iter().zip(params.iter()) {
                check_value(arg, return_type, debugger);

                if !is_assignable_from(param, &arg.typ) {
                    let expected = format!("{:?}", param);
                    let found = format!("{:?}", arg.typ);

                    debugger.throw_single(ErrorCode::ExpectedFound(expected, found), &arg.span);
                }
            }
        }

        ValueKind::InstanceMethod { reciever, .. } => {
            check_value(reciever.as_ref(), return_type, debugger);
        }

        ValueKind::InstanceVariable { reciever, .. } => {
            check_value(reciever.as_ref(), return_type, debugger);
        }

        ValueKind::If(if_value) => {
            let ty = value.typ.clone();

            check_if_value(if_value, vec![], &ty, debugger);
        }

        _ => {}
    }
}

fn check_if_value(if_value: &IfValue, mut spans: Vec<Span>, ty: &Type, debugger: &mut Debugger) {
    check_value(if_value.condition.as_ref(),
                &if_value.condition.typ,
                debugger);

    // Get the value of the if

    check_codeblock(&if_value.positive, ty, debugger);

    if !is_assignable_from(ty, &if_value.positive.typ()) {
        // Should be an error
        if let Some(span) = if_value.positive.span() {
            spans.push(span.clone());
        }
    }

    match &if_value.negative {
        Some(IfBranch::CodeBlock(negative)) => {
            check_codeblock(negative, ty, debugger);

            if !is_assignable_from(ty, &negative.typ()) {
                // Should be an error
                if let Some(span) = negative.span() {
                    spans.push(span.clone());
                }
            }
        }
        Some(IfBranch::Else(else_if)) => return check_if_value(else_if, spans, ty, debugger),
        None => {}
    }

    if spans.len() > 0 {
        debugger.throw(ErrorCode::MismatchedIfBranchTypes, spans);
    }
}

/// Whether ty1 can be assigned from ty2
pub fn is_assignable_from(ty1: &Type, ty2: &Type) -> bool {
    if ty2.kind() == &TypeKind::Divergent {
        return true;
    }

    if ty1.kind() == ty2.kind() {
        return true;
    }

    match (ty1.kind(), ty2.kind()) {
        _ => false,
    }
}
