use blir::{attributes::Attributes,
           code::{CodeBlock, FuncParam, Method, StatementKind},
           typ::{StructRef, TypeKind},
           value::ValueKind,
           Visibility};
use errors::Span;

pub(crate) fn add_default_initializer(r#struct: &StructRef) {
    // Get the variables
    let struct_variables = r#struct.borrow().instance_vars.clone();

    let self_type = TypeKind::Struct(r#struct.clone()).anon();

    // For now, create the value which we return
    // In the future, this function will take a reference
    // and initialize it
    let mut parameter_types = vec![];
    let mut parameter_labels = vec![];

    let mut init_statements = vec![];

    // Create a value for self
    let self_value = ValueKind::SelfVal(true).anon(self_type.clone());

    // Loop through the variables
    for variable in struct_variables {
        let variable_type = variable.borrow().typ.clone();
        let variable_name = variable.borrow().name.clone();
        let variable_default = variable.borrow().default_value.clone();

        // If the variable has a default value, assign it to the instance
        // Otherwise, give the function a parameter
        // and assign the parameter to self
        let assigned_value = if let Some(default_value) = variable_default {
            default_value
        } else {
            let parameter_value = ValueKind::FunctionParam(variable_name.clone()).anon(variable_type.clone());

            parameter_types.push(variable_type.clone());
            parameter_labels.push(variable_name);

            parameter_value
        };

        // Do an assignment
        let member = ValueKind::InstanceVariable { reciever: Box::new(self_value.clone()),
                                                   var:      variable, }.anon(variable_type);
        let assignment = ValueKind::Assign(Box::new(member), Box::new(assigned_value)).anon(TypeKind::Void.anon());

        init_statements.push(StatementKind::Eval { value:   assignment,
                                                   escaped: true, }.anon());
    }

    let code = CodeBlock::new(init_statements, Span::empty());
    let func_params = parameter_types.into_iter()
                                     .zip(parameter_labels)
                                     .map(|(typ, label)| FuncParam { label: Some(label.clone()),
                                                                          bind_name: label,
                                                                          typ,
                                                                          is_shared: false })
                                     .collect();

    let method = {
        let borrowed = r#struct.borrow();
        let scope = borrowed.scope();
        let path = borrowed.path();

        Method::new(Attributes::new(std::iter::empty()),
                    self_type.clone(),
                    false,
                    false,
                    true,
                    Visibility::Public,
                    "init".to_string(),
                    func_params,
                    TypeKind::Void.anon(),
                    code,
                    Span::empty(),
                    scope,
                    path)
    };

    {
        let mut borrowed = method.borrow_mut();
        let mangle_name = borrowed.mangle();
        borrowed.info.set_link_name(mangle_name);
        borrowed.add_params();
    }

    r#struct.add_method(method);
}
