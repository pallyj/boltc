use std::{io, cell::RefCell};

use blir::attributes::{AttributeFactory, FuncAttribute, AttributeArgs};
use bolt_ext::{Fix, FunctionSignature, FunctionKind};
use errors::{Span, DiagnosticReporter, IntoDiagnostic, Diagnostic, DiagnosticLevel, CodeLocation};
use parser::{operators::{OperatorFactory, OperatorFix}};

pub struct ExtensionHost {
	libraries: Vec<libloading::Library>,
	pub operator_factory: OperatorFactory,
	pub attribute_factory: AttributeFactory,
}

impl ExtensionHost {
	pub fn new() -> Self {
		let mut operator_factory = OperatorFactory::new();
		let attribute_factory = blir::attributes::default_attributes();

		operator_factory.register_intrinsics();

		Self {
			attribute_factory,
			libraries: vec![],
			operator_factory,
		}
	}

	pub unsafe fn load_extension(
		&mut self,
		path: &str) -> io::Result<()>
	{
		let extension_library = libloading::Library::new(path)
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
		let extension_description = extension_library
			.get::<*mut bolt_ext::ExtensionDescription>(b"extension_declaration\0")
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
			.read();

		self.libraries.push(extension_library);

		if extension_description.host_version() != bolt_ext::HOST_VERSION ||
		   extension_description.rustc_version() != bolt_ext::RUSTC_VERSION {
			// Throw an error
			return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported extension version"))
		}

		extension_description.register(self);

		Ok(())
	}
}

impl bolt_ext::ExtensionHost for ExtensionHost {
    fn register_operator(
		&mut self,
		operator: bolt_ext::Operator)
	{
		let fix = match operator.fix {
			Fix::Infix => OperatorFix::Infix,
			Fix::Prefix => OperatorFix::Prefix,
			Fix::Postfix => OperatorFix::Postfix,
		};

		let precedence = unsafe { std::mem::transmute(operator.precedence) };

		let operator = parser::operators::Operator::new(operator.name, operator.symbol, fix, precedence);

		self.operator_factory
			.register(operator)
    }

    fn register_attribute(
		&mut self,
		attribute: Box<dyn bolt_ext::Attribute>) {
        self.attribute_factory.register_func_attribute(AnyFuncAttribute { attribute: RefCell::new(attribute) })
    }
}

struct AnyFuncAttribute {
	attribute: RefCell<Box<dyn bolt_ext::Attribute>>
}

impl FuncAttribute for AnyFuncAttribute {
    fn name(&self) -> &'static str {
        self.attribute.borrow().label()
    }

    fn apply(&self, args: &AttributeArgs, info: &mut blir::code::FunctionInfo, _context: &mut blir::BlirContext, debugger: &mut errors::DiagnosticReporter) {
		let mut inline = false;
		let name = info.name().clone();
		let mut link_name = info.link_name().clone();
		let n_pars = info.params().len();
		let kind = if info.is_method() {
			FunctionKind::Method
		} else {
			FunctionKind::Function
		};

		let func_sig = FunctionSignature::new(&mut inline,
											  &name,
											  &mut link_name,
											  kind,
											n_pars);

		let function_span = info.span();
		let mut debugger = SimpleDebugger { span: function_span, debugger  };

        self.attribute
			.borrow_mut()
			.apply_to_func(func_sig, &mut debugger);

		info.set_link_name(link_name);
    }
}

pub struct SimpleDebugger<'a, 'b> {
	span: Span,
	debugger: &'a mut DiagnosticReporter<'b>
}

impl<'a, 'b> bolt_ext::Debugger for SimpleDebugger<'a, 'b> {
    fn warn(
		&mut self,
		warning: &str)
	{
		self.debugger.throw_diagnostic(Warning(String::from(warning), self.span));
    }

    fn throw(
		&mut self, 
		error: &str)
	{
		self.debugger.throw_diagnostic(Error(String::from(error), self.span));
    }
}

struct Error(String, Span);

impl IntoDiagnostic for Error {
    fn into_diagnostic(self) -> errors::Diagnostic {
        Diagnostic::new(DiagnosticLevel::Error,
						"attribute",
						self.0,
						vec![ CodeLocation::new(self.1, None) ])
    }
}

struct Warning(String, Span);

impl IntoDiagnostic for Warning {
    fn into_diagnostic(self) -> errors::Diagnostic {
        Diagnostic::new(DiagnosticLevel::Warning,
						"attribute",
						self.0,
						vec![ CodeLocation::new(self.1, None) ])
    }
}

pub enum ExtensionError {
	LoadFailed(String)
}

impl IntoDiagnostic for ExtensionError {
    fn into_diagnostic(self) -> Diagnostic {
		match self {
			Self::LoadFailed(name) => {
				Diagnostic::new(DiagnosticLevel::Error,
					"ext_load_failed",
					format!("extension {name} failed to load"),
					vec![])
			}
		}
    }
}