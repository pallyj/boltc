use std::{cell::{RefCell, Ref, RefMut}, sync::{Arc, Weak}, ops::Deref, collections::HashMap};

use mangle::{Path, MangledEnum};

use crate::{attributes::{Attributes}, Visibility, scope::ScopeRef, code::MethodRef, Symbol, SymbolWrapper};

use super::{TypeKind, CaseRef, Type, StructRef};

pub struct Enum {
	inner: RefCell<EnumInner>,
}

impl Enum {
	pub fn new(
		attributes: Attributes,
		visibility: Visibility,
		name:		String,
		parent:		&ScopeRef,
		parent_path:&Path,
		repr_type:	Type,
		meta: 		String) -> EnumRef
	{
		let arc = Arc::new_cyclic(|enum_arc| {
			let enum_inner = EnumInner {
				attributes,
				visibility,
				link_name: 		name.clone(),
				scope: 			parent.clone(),
				path: 			parent_path.clone().append(&name),
				name,
				methods:    	Vec::new(),
				cases: 	    	Vec::new(),
				substructs: 	Vec::new(),
				subenums:   	Vec::new(),
				named_variants: HashMap::new(),
				repr_type,
				self_ref: 		enum_arc.clone(),
				meta };

			Enum { inner: RefCell::new(enum_inner) }
		});

		let enum_ref = EnumRef { enum_ref: arc };

		enum_ref.inner.borrow().scope.add_symbol(String::from("Self"), Visibility::Public, Symbol::Type(TypeKind::Enum(enum_ref.clone())));

		enum_ref
	}
	pub fn attributes(
		&self) -> Ref<Attributes>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.attributes)
	}

	pub fn visibility(
		&self) -> Visibility
	{
		self.inner.borrow().visibility
	}

	pub fn name(
		&self) -> &str
	{
		&unsafe { &*self.inner.as_ptr() }.name
	}

	pub fn link_name(
		&self) -> &str
	{
		&unsafe { &*self.inner.as_ptr() }.link_name
	}

	pub fn set_link_name(
		&self,
		link_name: String)
	{
		self.inner.borrow_mut().link_name = link_name
	}

	pub fn path(
		&self) -> Path
	{
		self.inner.borrow().path.clone()
	}

	pub fn scope(
		&self) -> ScopeRef
	{
		self.inner.borrow().scope.clone()
	}

	pub fn add_method(
		&self,
		method: MethodRef)
	{
		// Add the function to the list of functions
        self.inner.borrow_mut().methods.push(method.clone());

        // Add the functions symbol, returning another symbol if it exists
        let name = if method.is_operator() {
            format!("op~{}", method.name())
        } else {
            method.name()
        };

        self.inner.borrow().scope.add_method(name, method);
	}

	pub fn methods(
		&self) -> Ref<Vec<MethodRef>>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.methods)
	}

	pub fn add_cases(
		&self,
		cases: Vec<CaseRef>)
	{
		let mut inner = self.inner.borrow_mut();

		for case in &cases {
			inner.named_variants.insert(case.name().clone(), case.clone());

			// TODO: Have a bool determining if it is a function or not
			let self_enum = EnumRef {enum_ref: inner.self_ref.upgrade().unwrap() };

			let sym = Symbol::EnumCase(self_enum.clone(), case.clone());

			inner.scope
			     .add_symbol(case.name().clone(), Visibility::Public, sym);
		}

		inner.cases.extend(cases);

		// TODO: Add a case value
	}

	pub fn add_substruct(&self, substruct: StructRef) -> Option<SymbolWrapper> {
        // Add the substruct to the list of substructs
        self.inner.borrow_mut().substructs.push(substruct.clone());

        // Add the substructs symbol, returning another symbol if it exists
        let visibility = substruct.visibility();
        let name = substruct.name();

        let symbol = Symbol::Type(TypeKind::Struct(substruct));

        self.inner.borrow().scope.add_symbol(name, visibility, symbol)
    }

	pub fn substructs(
		&self) -> Ref<Vec<StructRef>>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.substructs)
	}

    pub fn add_subenum(&self, subenum: EnumRef) -> Option<SymbolWrapper> {
        // Add the substruct to the list of substructs
        self.inner.borrow_mut().subenums.push(subenum.clone());

        // Add the substructs symbol, returning another symbol if it exists
        let visibility = subenum.visibility();
        let name = subenum.name().to_string();

        let symbol = Symbol::Type(subenum.get_type());

        self.inner.borrow().scope.add_symbol(name, visibility, symbol)
    }

	pub fn subenums(
		&self) -> Ref<Vec<EnumRef>>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.subenums)
	}

	pub fn add_type(&self, name: String, visibility: Visibility, typ: TypeKind) -> Option<SymbolWrapper> {
        let sym = Symbol::Type(typ);

        self.inner.borrow().scope.add_symbol(name, visibility, sym)
    }

	pub fn get_variant(
		&self,
		name: &str) -> Option<CaseRef>
	{
		self.inner.borrow()
			.named_variants.get(name)
			.cloned()
	}

	pub fn variants(
		&self) -> Ref<Vec<CaseRef>>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.cases)
	}

	pub fn repr_type(
		&self) -> Ref<Type>
	{
		Ref::map(self.inner.borrow(), |inner| &inner.repr_type)
	}

	pub fn repr_type_mut(
		&self) -> RefMut<Type>
	{
		RefMut::map(self.inner.borrow_mut(), |inner| &mut inner.repr_type)
	}

	pub fn mangle(&self) -> String { MangledEnum(&self.inner.borrow().path).to_string() }

	pub fn lookup_static_item(&self, name: &str) -> Option<Symbol> {
		self.scope()
			.lookup_static_member(name)
			.map(|wrapper| wrapper.resolve())
	}

	pub fn meta(&self) -> String {
		self.inner.borrow().meta.clone()
	}
}

struct EnumInner {
	pub attributes: Attributes,
	pub visibility: Visibility,

	pub name:		String,
	pub link_name:	String,

	scope: 			ScopeRef,
	path:			Path,

	methods:		Vec<MethodRef>,
	cases:			Vec<CaseRef>,
	substructs:		Vec<StructRef>,
	subenums:		Vec<EnumRef>,

	named_variants: HashMap<String, CaseRef>,

	repr_type:		Type,

	self_ref:		Weak<Enum>,

	meta: 			String,
}

#[derive(Clone)]
pub struct EnumRef {
	enum_ref: Arc<Enum>
}

impl EnumRef {
	pub fn get_type(&self) -> TypeKind {
		TypeKind::Enum(self.clone())
	}
}

impl Deref for EnumRef {
    type Target = Enum;

    fn deref(&self) -> &Self::Target {
        &self.enum_ref
    }
}

impl std::hash::Hash for EnumRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.enum_ref.name().hash(state);
        self.enum_ref.path().hash(state);
    }
}

impl PartialEq for EnumRef {
    fn eq(&self, other: &Self) -> bool { Arc::ptr_eq(&self.enum_ref, &other.enum_ref) }
}

impl Eq for EnumRef {
    fn assert_receiver_is_total_eq(&self) {}
}

impl std::fmt::Debug for EnumRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} enum {} {{", self.visibility(), self.name())?;

		for variant in self.inner.borrow().cases.iter() {
			writeln!(f, "\t{}", format!("{variant:?}").replace("\n", "\n\t"))?;
		}

        /*for var in self.borrow().instance_vars.iter() {
            writeln!(f, "\t{}", format!("{var:?}").replace("\n", "\t"))?;
        }

        for substruct in self.borrow().substructs.iter() {
            writeln!(f, "\t{}", format!("{substruct:?}").replace("\n", "\t"))?;
        }

        for func in self.borrow().methods.iter() {
            writeln!(f, "\t{}", format!("{func:?}").replace("\n", "\n\t"))?;
        }*/

        write!(f, "}}")
    }
}
