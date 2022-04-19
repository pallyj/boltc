use blir::{code::{CodeBlock, FunctionRef, MethodRef, Statement, StatementKind},
           typ::{StructRef, Type, TypeKind},
           value::{IfBranch, IfValue, Value, ValueKind, VarRef, ConstantRef},
           Library};
use errors::{debugger::Debugger, error::ErrorCode, Span};

pub struct TypeCheckPass<'a, 'b> {
    debugger: &'a mut Debugger<'b>,
}

// TODO: Error for non-existant attributes
impl<'a, 'b> TypeCheckPass<'a, 'b> {
    pub fn new(debugger: &'a mut Debugger <'b>) -> Self {
        Self {
            debugger
        }
    }

    pub fn run_pass(&mut self, library: &mut Library) {
        for func in &library.functions {
            self.check_function(func);
        }

        for r#struct in &library.structs {
            self.check_struct(r#struct);
        }

        for constant in &library.constants {
            self.check_const(constant);
        }
    }

    fn check_struct(&mut self, r#struct: &StructRef) {
        let struct_ref = r#struct.borrow();

        for substruct in &struct_ref.substructs {
            self.check_struct(substruct);
        }

        for method in &struct_ref.methods {
            self.check_method(method);
        }

        for instance_vars in &struct_ref.instance_vars {
            self.check_var(instance_vars);
        }

        for constant in &struct_ref.constants {
            self.check_const(constant);
        }
    }

    fn check_function(&mut self, func: &FunctionRef) {
        let borrowed = func.borrow();
        let func_return_type = borrowed.info.return_type();
        let func_code = &borrowed.code;

        if let Err(error) = self.check_codeblock(func_code, func_return_type, func_return_type) {
            println!("Error: function");
        }
    }

    fn check_method(&mut self, func: &MethodRef) {
        let borrowed = func.borrow();
        let func_return_type = borrowed.info.return_type();
        let func_code = &borrowed.code;

        if let Err(error) = self.check_codeblock(func_code, func_return_type, func_return_type) {
            println!("Error: function");
        }
    }

    fn check_var(&mut self, var: &VarRef) {
        let variable = var.borrow();

        let Some(value) = &variable.default_value else { return };

        if let Err(error) = self.check_type(&variable.typ, &value.typ) {
            self.handle_var_error(error, &variable.span);
        }
    }

    fn check_const(&mut self, var: &ConstantRef) {
        let variable = var.borrow();

        if let Err(error) = self.check_type(&variable.typ, &variable.value.typ) {
            self.handle_var_error(error, &variable.span);
        }
    }

    fn check_codeblock(
        &mut self,
        code_block: &CodeBlock,
        code_block_type: &Type,
        return_type: &Type) -> Result<(), TypeCheckError>
    {
        for smt in code_block.statements() {
            self.check_smt(smt, return_type);
        }

        // TODO: Handle the error
        self.check_type(code_block_type, &code_block.typ())
    }

    fn check_smt(
        &mut self,
        statement: &Statement,
        return_type: &Type)
    {
        match &statement.kind {
            StatementKind::Eval { value, .. } => self.check_value(value, return_type),
            StatementKind::Bind { name, typ, value, .. } => {
                let Some(value) = value else { return };

                if let Err(error) = self.check_type(typ, &value.typ) {
                    self.handle_let_error(error, statement);
                }
            }
            StatementKind::Return { value } => {
                let check_result =
                if let Some(value) = value {
                    self.check_type(return_type, &value.typ)
                } else {
                    self.check_type(return_type, &TypeKind::Void.anon())
                };

                if let Err(error) = check_result {
                    self.handle_return_error(error, statement);
                }
            }
        }
    }

    fn check_value(
        &mut self,
        value: &Value,
        return_type: &Type)
    {
        // TODO: Move this to another function
        match value.typ.kind() {
            TypeKind::Error => println!("Error: error type"),

            TypeKind::SomeBool |
            TypeKind::SomeInteger |
            TypeKind::SomeFloat | 
            TypeKind::SomeFunction |
            TypeKind::Infer { .. } => println!("Error: couldn't infer"),

            TypeKind::Named(_) => println!("Error: named type"),
            TypeKind::Member { .. } => println!("Error: member type"),

            _ => {}
        }

        match &value.kind {
            ValueKind::Assign(place, value) => {
                self.check_value(place, return_type);
                self.check_value(value, return_type);

                // TODO: Check that they have the same type

            }

            ValueKind::Closure(closure) => {
                let TypeKind::Function { return_type, .. } = value.typ.kind() else {
                    println!("Error: not a function");
                    return
                };

                if let Err(error) = self.check_codeblock(&closure.code, return_type, return_type) {
                    println!("Error: handle closure error");
                }
            }
            ValueKind::FuncCall { function, args } => {
                self.check_value(&function, return_type);

                let params = match &function.typ.kind {
                    TypeKind::Function { params, .. } => params,
                    TypeKind::Method { params, .. } => params,
                    _ => {
                        println!("Error: not a function");
                        return
                    }
                };

                if params.len() != args.args.len() {
                    println!("ICE");
                }

                for (param, arg) in params.iter().zip(&args.args) {
                    self.check_value(arg, return_type);

                    if let Err(error) = self.check_type(param, &arg.typ) {
                        self.handle_call_error(error, arg);
                    }
                }
            }
            ValueKind::If(if_value) => self.check_if_value(if_value, &value.typ, return_type, vec![]),
            ValueKind::InstanceMethod { reciever, .. } => self.check_value(reciever, return_type),
            ValueKind::InstanceVariable { reciever, .. } => self.check_value(reciever, return_type),

            ValueKind::Error => println!("Error: error"),
            ValueKind::Member { .. } => println!("Error: member"),
            ValueKind::Named(_) => println!("Error: named"),
            ValueKind::Operator(_) => println!("Error: operator"),
            ValueKind::Polymorphic(polymorphic) => println!("Error: poly"),
            ValueKind::PolymorphicMethod { polymorphic, .. } => println!("Error: poly"),

            _ => { /* Do nothing */ }
        }
    }

    fn check_if_value(
        &mut self,
        if_value: &IfValue,
        if_type: &Type,
        return_type: &Type,
        mut spans: Vec<Span>)
    {
        self.check_value(if_value.condition.as_ref(),
                         &if_value.condition.typ);
    
        // Get the value of the if
        if let Err(_) = self.check_codeblock(&if_value.positive, if_type, return_type) {
            spans.extend(if_value.positive.span());
        }

        if let Some(negative) = &if_value.negative {
            match negative {
                IfBranch::CodeBlock(negative_block) => {
                    if let Err(_) = self.check_codeblock(negative_block, if_type, return_type) {
                        spans.extend(if_value.positive.span());
                    }
                }
                IfBranch::Else(else_if_branch) => return self.check_if_value(if_value, if_type, return_type, spans),
            }
        }
    
        if !spans.is_empty() {
            self.debugger.throw(ErrorCode::MismatchedIfBranchTypes, spans);
        }
    }

    fn check_type(
        &self,
        place: &Type,
        value: &Type) -> Result<(), TypeCheckError>
    {
        match (place.kind(), value.kind()) {
            (_, TypeKind::Infer { .. }) | (TypeKind::Infer { .. }, _) => Err(TypeCheckError::CouldNotInfer),

            (ty1, ty2) if ty1 == ty2 => Ok(()),
            (_, TypeKind::Divergent) => Ok(()),

            _ => Err(TypeCheckError::MismatchedTypes(place.clone(), value.clone()))
        }
    }

    fn handle_let_error(
        &mut self,
        error: TypeCheckError,
        statement: &Statement)
    {
        println!("Error: let");
    }

    fn handle_var_error(
        &mut self,
        error: TypeCheckError,
        span: &Span)
    {
        match error {
            TypeCheckError::CouldNotInfer => {
                self.debugger.throw(ErrorCode::AmbiguousTy, vec![span.clone()])
            }
            TypeCheckError::MismatchedTypes(t1, t2) => {
                println!("Mismatched types {t1:?} {t2:?}")
            }
        }
    }

    fn handle_return_error(
        &mut self,
        error: TypeCheckError,
        statement: &Statement)
    {
        println!("Error: return");
    }

    fn handle_call_error(
        &mut self,
        error: TypeCheckError,
        arg: &Value)
    {
        println!("Error: call");
    }
}

enum TypeCheckError {
    CouldNotInfer,
    MismatchedTypes(Type, Type)
}