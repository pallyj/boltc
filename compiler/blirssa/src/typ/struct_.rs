use std::{cell::{Ref, RefCell},
          collections::HashMap,
          fmt::Display,
          ops::Deref,
          sync::Arc};

use crate::typ::Type;

#[derive(Clone, PartialEq, Eq)]
pub struct StructRef {
    r#struct: Arc<Struct>,
}

impl Deref for StructRef {
    type Target = Struct;

    fn deref(&self) -> &Self::Target { &self.r#struct }
}

impl StructRef {
    pub fn typ(&self) -> Type { Type::Struct { container: self.clone() } }
}

impl Display for StructRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.r#struct) }
}

#[derive(PartialEq, Eq)]
pub struct Struct {
    name:           String,
    is_transparent: bool,
    is_packed:      bool,
    fields:         RefCell<Vec<StructField>>,
    field_indices:  RefCell<HashMap<String, u32>>,
}

#[derive(PartialEq, Eq)]
pub struct StructField {
    name: String,
    typ:  Type,
}

impl Struct {
    pub fn new(name: String, is_transparent: bool, is_packed: bool) -> StructRef {
        StructRef { r#struct: Arc::new(Struct { name,
                                                is_transparent,
                                                is_packed,
                                                fields: RefCell::new(Vec::new()),
                                                field_indices: RefCell::new(HashMap::new()) }), }
    }

    pub fn fields(&self) -> Ref<Vec<StructField>> { self.fields.borrow() }

    pub fn add_field(&self, field: StructField) {
        let mut fields = self.fields.borrow_mut();

        self.field_indices
            .borrow_mut()
            .insert(field.name.clone(), fields.len() as u32);
        fields.push(field);
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn transparent_type(&self) -> Option<Type> {
        if self.is_transparent {
            self.fields
                .borrow()
                .first()
                .map(|field| &field.typ)
                .cloned()
        } else {
            None
        }
    }

    pub fn is_packed(&self) -> bool { self.is_packed }

    pub fn get_field_type(&self, name: &str) -> Type {
        let idx = *self.field_indices
                       .borrow()
                       .get(name)
                       .expect("Field doesn't exist") as usize;

        self.fields.borrow()[idx].typ.clone()
    }

    pub fn get_field_index(&self, name: &str) -> u32 {
        *self.field_indices
             .borrow()
             .get(name)
             .expect("Field doesn't exist")
    }
}

impl StructField {
    pub fn new(name: &str, typ: Type) -> StructField {
        StructField { name: name.to_string(),
                      typ }
    }

    pub fn typ_ref(&self) -> &Type { &self.typ }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "struct {} {{", self.name)?;

        for field in self.fields.borrow().iter() {
            writeln!(f, "    {}: {}", field.name, field.typ)?;
        }

        writeln!(f, "}}")
    }
}
