use std::{collections::HashMap, fmt::format};

use errors::{error::ErrorCode, Span, DiagnosticReporter, IntoDiagnostic, Diagnostic, DiagnosticLevel, CodeLocation};
use itertools::Itertools;

use crate::{code::FunctionInfo, typ::{StructRef, Type, TypeKind}, BlirContext, value::FunctionArgs};

#[derive(Clone)]
pub struct Attribute {
    attribute_name: String,
    args:           AttributeArgs,
    span:           Span,
}

impl Attribute {
    pub fn new(attribute_name: String, args: AttributeArgs, span: Span) -> Self { Self { attribute_name, args, span } }

    pub fn name(&self) -> &str { &self.attribute_name }

    pub fn span(&self) -> Span { self.span }
}

#[derive(Clone)]
pub struct Attributes {
    attributes: Vec<Attribute>,
}

impl Attributes {
    pub fn new(attributes: impl Iterator<Item = Attribute>) -> Self { Self { attributes: attributes.collect(), } }

    pub fn iter(&self) -> impl Iterator<Item = &Attribute> { self.attributes.iter() }
}

pub struct AttributeFactory {
    func_attributes:   HashMap<String, Box<dyn FuncAttribute>>,
    struct_attributes: HashMap<String, Box<dyn StructAttribute>>,
}

impl AttributeFactory {
    pub fn new() -> Self {
        Self { struct_attributes: HashMap::new(),
               func_attributes:   HashMap::new(), }
    }

    pub fn register_struct_attribute<T: StructAttribute + 'static>(&mut self, attribute: T) {
        let name = attribute.name().to_string();
        let wrapped = Box::new(attribute);

        self.struct_attributes.insert(name, wrapped);
    }

    pub fn register_func_attribute<T: FuncAttribute + 'static>(&mut self, attribute: T) {
        let name = attribute.name().to_string();
        let wrapped = Box::new(attribute);

        self.func_attributes.insert(name, wrapped);
    }

    pub fn apply_struct_attributes(&self, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let attributes = struct_ref.borrow().attributes.clone();

        for attribute in attributes.iter() {
            if let Some(attr) = self.struct_attributes.get(attribute.name()) {
                attr.apply(&attribute.args, struct_ref, context, debugger);
            } else {
                // Throw an error
                let struct_span = Span::empty(); // todo: have a span
                debugger.throw_diagnostic(AttributeError::DoesNotExist(String::from(attribute.name())).at(attribute.span(), struct_span))
            }
        }
    }

    pub fn apply_func_attributes(&self, attributes: &Attributes, func_info: &mut FunctionInfo, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        for attribute in attributes.iter() {
            if let Some(attr) = self.func_attributes.get(attribute.name()) {
                attr.apply(&attribute.args, func_info, context, debugger);
            } else {
                // Throw an error
                let struct_span = func_info.span();
                debugger.throw_diagnostic(AttributeError::DoesNotExist(String::from(attribute.name())).at(attribute.span(), struct_span))
            }
        }
    }
}

pub fn default_attributes() -> AttributeFactory {
    let mut factory = AttributeFactory::new();

    factory.register_func_attribute(EntryPointAttribute {});
    factory.register_func_attribute(ExportCAttribute {});
    factory.register_func_attribute(HiddenFuncAttribute {});

    factory.register_struct_attribute(TransparentAttribute {});
    factory.register_struct_attribute(DefaultIntegerReprAttribute {});
    factory.register_struct_attribute(DefaultFloatReprAttribute {});
    factory.register_struct_attribute(DefaultBoolReprAttribute {});
    factory.register_struct_attribute(DefaultStringReprAttribute {});
    factory.register_struct_attribute(DefaultCharReprAttribute {});
    factory.register_struct_attribute(CharExpressibleAttribute {});

    factory
}

pub trait FuncAttribute {
    fn name(&self) -> &'static str;

    fn apply(&self, args: &AttributeArgs, info: &mut FunctionInfo, context: &mut BlirContext, debugger: &mut DiagnosticReporter);
}

pub trait StructAttribute {
    fn name(&self) -> &'static str;

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter);
}

struct EntryPointAttribute;
struct ExportCAttribute;
struct HiddenFuncAttribute;
struct ExternAttribute;

struct TransparentAttribute;
struct DefaultIntegerReprAttribute;
struct DefaultFloatReprAttribute;
struct DefaultBoolReprAttribute;

struct DefaultCharReprAttribute;

struct CharExpressibleAttribute;

struct DefaultStringReprAttribute;

impl FuncAttribute for EntryPointAttribute {
    fn name(&self) -> &'static str { "entryPoint" }

    fn apply(&self, args: &AttributeArgs, info: &mut FunctionInfo, context: &mut BlirContext, _debugger: &mut DiagnosticReporter) {
        let _ = context.entry_point.insert(info.link_name().clone());
    }
}

impl FuncAttribute for ExportCAttribute {
    fn name(&self) -> &'static str { "exportC" }

    fn apply(&self, args: &AttributeArgs, info: &mut FunctionInfo, _context: &mut BlirContext, _debugger: &mut DiagnosticReporter) { info.set_link_name(info.name().clone()) }
}

impl FuncAttribute for HiddenFuncAttribute {
    fn name(&self) -> &'static str { "hidden" }

    fn apply(&self, args: &AttributeArgs, info: &mut FunctionInfo, _context: &mut BlirContext, _debugger: &mut DiagnosticReporter) { info.hide() }
}

impl FuncAttribute for ExternAttribute {
    fn name(&self) -> &'static str {
        "extern"
    }

    fn apply(&self, args: &AttributeArgs, info: &mut FunctionInfo, _context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        if args.count() == 0 {
            // extern "C"
            let link_name = info.name().clone();
            info.set_link_name(link_name)
        } else if args.count() == 1 {
            // check the label
            if let Some(label) = args.get_label(0) {
                let val = args.get_indexed(0).unwrap();
                match label.as_str() {
                    "linkName" => match val {
                        AttributeArg::String(link_name) => {
                            info.set_link_name(link_name.clone())
                        },
                        _ => {
                            // Error
                            debugger.throw_diagnostic(AttributeError::WrongArgType.at(Span::empty(), info.span()))
                        }
                    },
                    _ => {}
                }
            } else {
                // error
                debugger.throw_diagnostic(AttributeError::WrongLabel(String::from("_")).at(Span::empty(), info.span()))
            }
        } else {
            debugger.throw_diagnostic(AttributeError::ExpectedOneArg.at(Span::empty(), info.span()))
        }
    }
}


impl StructAttribute for TransparentAttribute {
    fn name(&self) -> &'static str { "transparent" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, _context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if struct_ref.borrow().instance_vars.len() != 1 {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::TransparentOneVar { struct_name: struct_ref.name(),
                                                                          var_spans: struct_ref.borrow().instance_vars.iter().map(|v| v.borrow().span).collect_vec() }.at(attribute_span, struct_span));
        }
        struct_ref.borrow_mut().is_transparent = true;
    }
}

impl StructAttribute for DefaultIntegerReprAttribute {
    fn name(&self) -> &'static str { "defaultIntegerRepr" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if !struct_ref.integer_repr() {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::NotExpressible {
                struct_name: struct_ref.name(),
                as_primitive: "integer"
            }.at(attribute_span, struct_span))
        }

        context.default_integer_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultFloatReprAttribute {
    fn name(&self) -> &'static str { "defaultFloatRepr" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if !struct_ref.float_repr() {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::NotExpressible {
                struct_name: struct_ref.name(),
                as_primitive: "float"
            }.at(attribute_span, struct_span))
        }

        context.default_float_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultBoolReprAttribute {
    fn name(&self) -> &'static str { "defaultBooleanRepr" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if !struct_ref.bool_repr() {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::NotExpressible {
                struct_name: struct_ref.name(),
                as_primitive: "bool"
            }.at(attribute_span, struct_span))
        }

        context.default_bool_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultStringReprAttribute {
    fn name(&self) -> &'static str { "defaultStringRepr" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if !struct_ref.string_repr() {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::NotExpressible {
                struct_name: struct_ref.name(),
                as_primitive: "string"
            }.at(attribute_span, struct_span))
        }

        context.default_string_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultCharReprAttribute {
    fn name(&self) -> &'static str { "defaultCharRepr" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if !struct_ref.char_repr() {
            // Throw an error
            debugger.throw_diagnostic(AttributeError::NotExpressible {
                struct_name: struct_ref.name(),
                as_primitive: "char"
            }.at(attribute_span, struct_span))
        }

        context.default_char_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for CharExpressibleAttribute {
    fn name(&self) -> &'static str { "charExpressible" }

    fn apply(&self, args: &AttributeArgs, struct_ref: &StructRef, _context: &mut BlirContext, debugger: &mut DiagnosticReporter) {
        let struct_span = Span::empty();
        let attribute_span = Span::empty();

        if struct_ref.borrow().instance_vars.len() != 1 {
            debugger.throw_diagnostic(AttributeError::TooManyVars {
                struct_name: struct_ref.name(),
                primitive_name: "char",
                var_spans: struct_ref.borrow().instance_vars.iter().map(|v| v.borrow().span).collect_vec()

            }.at(attribute_span, struct_span))
        }
        struct_ref.borrow_mut().is_char_repr = true;
    }
}

enum AttributeError {
    DoesNotExist(String),

    NotExpressible { struct_name: String, as_primitive: &'static str },
    TooManyVars { struct_name: String, primitive_name: &'static str, var_spans: Vec<Span> },
    WrongVarType {
        struct_name: String,

        var_type: Type,
        var_span: Span,

        primitive_name: &'static str,
    },
    TransparentOneVar { struct_name: String, var_spans: Vec<Span> },

    ExpectedOneArg,

    WrongLabel(String),

    WrongArgType,
}

impl AttributeError {
    pub fn at(self, attribute_span: Span, struct_span: Span) -> AttributeErrorSpanned {
        AttributeErrorSpanned { error: self, attribute_span, struct_span }
    }
}

struct AttributeErrorSpanned {
    error: AttributeError,
    attribute_span: Span,
    struct_span: Span,
}

impl IntoDiagnostic for AttributeErrorSpanned {
    fn into_diagnostic(self) -> errors::Diagnostic {
        match self.error {
            AttributeError::DoesNotExist(name) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "attribute_dne",
                                format!("Attribute {name} does not exist"),
                                vec![
                                    CodeLocation::new(self.attribute_span, Some(String::from("remove the attribute")))
                                ])
            }
            AttributeError::TransparentOneVar { struct_name, var_spans } => {
                let vsl = var_spans.len();
                let locs = std::iter::once(CodeLocation::new(self.attribute_span, Some(String::from("remove this attribute"))))
                    .chain(var_spans.into_iter().map(|span| CodeLocation::new(span, None)))
                    .collect();

                Diagnostic::new(DiagnosticLevel::Error,
                                "too_many_vars",
                                format!("transparent structs need one field, this one has {}", vsl),
                                locs)
            }
            AttributeError::WrongVarType { struct_name, var_type, var_span, primitive_name } => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "prim_wrong_ty",
                                format!("{{{primitive_name}}} primitive can't be represented by {var_type}"),
                                vec![
                                    CodeLocation::new(self.attribute_span, None),
                                    CodeLocation::new(var_span, Some(format!("change type to {{{primitive_name}}}"))),
                                ])
            }
            AttributeError::NotExpressible { struct_name, as_primitive } => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "not_expressible",
                                format!("struct `{struct_name}` is not expressible as {as_primitive}"),
                                vec![
                                    CodeLocation::new(self.attribute_span, Some(String::from("remove this attribute"))),
                                    CodeLocation::new(self.struct_span, None),
                                ])
            }
            AttributeError::TooManyVars { struct_name, primitive_name, var_spans } => {
                let vsl = var_spans.len();
                let locs = std::iter::once(CodeLocation::new(self.attribute_span, Some(String::from("remove this attribute"))))
                    .chain(var_spans.into_iter().map(|span| CodeLocation::new(span, None)))
                    .collect();

                Diagnostic::new(DiagnosticLevel::Error,
                                "too_many_vars",
                                format!("{{{primitive_name}}} structs need one field, this one has {}", vsl),
                                locs)
            }
            AttributeError::ExpectedOneArg => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "expected_one_arg",
                                format!("expected one arg in attribute"),
                                vec![ CodeLocation::new(self.struct_span, None) ])
            }
            AttributeError::WrongLabel(label) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "wrong_label",
                                format!("found incorrect label {label} in attribute"),
                                vec![ CodeLocation::new(self.struct_span, None) ])
            }
            AttributeError::WrongArgType => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "wrong_arg_type",
                                format!("attribute arg has the wrong type"),
                                vec![ CodeLocation::new(self.struct_span, None) ])
            }
        }
    }
}

#[derive(Clone)]
pub struct AttributeArgs {
    args: Vec<(Option<String>, AttributeArg)>,
}

#[derive(Clone)]
pub enum AttributeArg {
    Integer(u64),
    Float(f64),
    Bool(bool),
    String(String),
    Variant(String),
    Named(String),
}

impl AttributeArgs {
    pub fn new(args: Vec<(Option<String>, AttributeArg)>,) -> Self {
        Self { args }
    }

    ///
    /// Gets an arg for an attribute by name
    /// 
    pub fn get(&self, name: &str) -> Option<&AttributeArg> {
        self.args
            .iter()
            .find_map(|(label, arg)| label.is_some_and(|label| label == name).then_some(arg) )
    }

    ///
    /// Gets the argument of an attribute at an index
    /// 
    pub fn get_indexed(&self, idx: usize) -> Option<&AttributeArg> {
        if idx >= self.args.len() {
            return None;
        }

        Some(&self.args[idx].1)
    }

    ///
    /// Gets the label of an arg of an attribute at an index
    /// 
    pub fn get_label(&self, idx: usize) -> Option<&String> {
        if idx >= self.args.len() {
            return None;
        }

        self.args[idx].0.as_ref()
    }

    ///
    /// The number of args given to the argument
    /// 
    pub fn count(&self) -> usize {
        self.args.len()
    }
}