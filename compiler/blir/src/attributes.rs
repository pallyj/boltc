use std::collections::HashMap;

use errors::{debugger::Debugger, error::ErrorCode, Span};

use crate::{code::FunctionInfo, typ::StructRef, BlirContext};

#[derive(Clone)]
pub struct Attribute {
    attribute_name: String,
    span:           Span,
}

impl Attribute {
    pub fn new(attribute_name: String, span: Span) -> Self { Self { attribute_name, span } }

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

    pub fn apply_struct_attributes(&self, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut Debugger) {
        let attributes = struct_ref.borrow().attributes.clone();

        for attribute in attributes.iter() {
            if let Some(attribute) = self.struct_attributes.get(attribute.name()) {
                attribute.apply(struct_ref, context, debugger);
            } else {
                // Throw an error
                debugger.throw(ErrorCode::AttributeDoesNotExist(attribute.name().to_string()),
                               vec![attribute.span()]);
            }
        }
    }

    pub fn apply_func_attributes(&self, attributes: &Attributes, func_info: &mut FunctionInfo, context: &mut BlirContext, debugger: &mut Debugger) {
        for attribute in attributes.iter() {
            if let Some(attribute) = self.func_attributes.get(attribute.name()) {
                attribute.apply(func_info, context, debugger);
            } else {
                // Throw an error
                debugger.throw(ErrorCode::AttributeDoesNotExist(attribute.name().to_string()),
                               vec![attribute.span()]);
            }
        }
    }
}

pub fn default_attributes() -> AttributeFactory {
    let mut factory = AttributeFactory::new();

    factory.register_func_attribute(EntryPointAttribute {});
    factory.register_func_attribute(ExportCAttribute {});

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

    fn apply(&self, info: &mut FunctionInfo, context: &mut BlirContext, debugger: &mut Debugger);
}

pub trait StructAttribute {
    fn name(&self) -> &'static str;

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, debugger: &mut Debugger);
}

struct EntryPointAttribute;
struct ExportCAttribute;

struct TransparentAttribute;
struct DefaultIntegerReprAttribute;
struct DefaultFloatReprAttribute;
struct DefaultBoolReprAttribute;

struct DefaultCharReprAttribute;

struct CharExpressibleAttribute;

struct DefaultStringReprAttribute;

impl FuncAttribute for EntryPointAttribute {
    fn name(&self) -> &'static str { "entryPoint" }

    fn apply(&self, info: &mut FunctionInfo, context: &mut BlirContext, _debugger: &mut Debugger) { let _ = context.entry_point.insert(info.link_name().clone()); }
}

impl FuncAttribute for ExportCAttribute {
    fn name(&self) -> &'static str { "exportC" }

    fn apply(&self, info: &mut FunctionInfo, _context: &mut BlirContext, _debugger: &mut Debugger) { info.set_link_name(info.name().clone()) }
}

impl StructAttribute for TransparentAttribute {
    fn name(&self) -> &'static str { "transparent" }

    fn apply(&self, struct_ref: &StructRef, _context: &mut BlirContext, _debugger: &mut Debugger) {
        if struct_ref.borrow().instance_vars.len() != 1 {
            // Throw an error
        }
        struct_ref.borrow_mut().is_transparent = true;
    }
}

impl StructAttribute for DefaultIntegerReprAttribute {
    fn name(&self) -> &'static str { "defaultIntegerRepr" }

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, _debugger: &mut Debugger) {
        if !struct_ref.integer_repr() {
            // Throw an error
        }

        context.default_integer_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultFloatReprAttribute {
    fn name(&self) -> &'static str { "defaultFloatRepr" }

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, _debugger: &mut Debugger) {
        if !struct_ref.float_repr() {
            // Throw an error
        }

        context.default_float_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultBoolReprAttribute {
    fn name(&self) -> &'static str { "defaultBooleanRepr" }

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, _debugger: &mut Debugger) {
        if !struct_ref.bool_repr() {
            // Throw an error
        }

        context.default_bool_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultStringReprAttribute {
    fn name(&self) -> &'static str { "defaultStringRepr" }

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, _debugger: &mut Debugger) {
        if !struct_ref.string_repr() {
            // Throw an error
        }

        context.default_string_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for DefaultCharReprAttribute {
    fn name(&self) -> &'static str { "defaultCharRepr" }

    fn apply(&self, struct_ref: &StructRef, context: &mut BlirContext, _debugger: &mut Debugger) {
        if !struct_ref.string_repr() {
            // Throw an error
        }

        context.default_char_repr = Some(struct_ref.clone());
    }
}

impl StructAttribute for CharExpressibleAttribute {
    fn name(&self) -> &'static str { "charExpressible" }

    fn apply(&self, struct_ref: &StructRef, _context: &mut BlirContext, _debugger: &mut Debugger) {
        if struct_ref.borrow().instance_vars.len() != 1 {
            // Throw an error
        }
        struct_ref.borrow_mut().is_char_repr = true;
    }
}