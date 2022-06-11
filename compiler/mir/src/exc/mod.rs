use itertools::Itertools;

use crate::{Project, code::BasicBlockId, instr::{Terminator, Instruction}, val::{RValue, DuoIntrinsic, Place, SoloIntrinsic, ConstValue}, ty::{Type, TypeKind}};

use self::{val::Value, frame::StackFrame};

pub mod val;
mod var;
mod frame;

//
// todo:
//  - struct 
//  - enum
//  - references
//

pub struct ExecutionEngine<'a> {
	project: &'a Project
}

impl<'a> ExecutionEngine<'a> {
	pub (crate) fn new(project: &'a Project) -> Self {
		Self { project }
	}

	pub fn run_function(&self, name: &str, params: Vec<Value>) -> Value {
		let function = self.project.get_function_named(name).expect("Function does not exist");

		if function.params().len() != params.len() {
			panic!("Wrong number of params passed in {name}");
		}

		// Extract all mutable state into a StackFrame
		let mut stack_frame = StackFrame::new(function.params(), function.locals(), self.project);
		stack_frame.use_parameters(params);

		let mut basic_block_id = function.first_basic_block();

		loop {
			let basic_block = self.project.basic_block(basic_block_id)
										  .expect("basic block doesn't exist");

			for instruction in basic_block.instructions() {
				self.run_instruction(instruction, &mut stack_frame);
			}

			match self.next_after_terminator(basic_block.terminator(), &mut stack_frame) {
				Action::JumpTo(next_basic_block) => basic_block_id = next_basic_block,
				Action::Return(value) => return value
			}
		}
	}

	pub fn eval(&self, value: &RValue, frame: &mut StackFrame) -> Value {
		use crate::val::RValueKind::*;

		match value.kind() {
			Const(ConstValue::String(s)) => {
				let bytes = s.bytes().map(|byte| Value::Int(byte as u64)).chain(std::iter::once(Value::Int(0))).collect_vec().leak();

				Value::Ref(bytes.as_mut_ptr())
			}
			Const(const_value) => Value::from_const(const_value.clone()),
			Copy(place) => self.copy(&place, frame),
			Move(place) => self.move_value(&place, frame),
			DuoIntrinsic { intrinsic, left, right } => self.duo_intrinsic(*intrinsic, self.eval(&left, frame), self.eval(&right, frame)),
			SoloIntrinsic { intrinsic, operand } => self.solo_intrinsic(*intrinsic, self.eval(operand, frame)),
			Ref(place) => {
				let ptr_self = self.get_mut_ptr(place, frame) as *mut Value;

				Value::Ref(ptr_self)
			},
			Call { function, params } => {
				let function = self.eval(function, frame);
				let params = params.iter().map(|param| self.eval(param, frame)).collect_vec();

				use Value::*;

				match function {
					Function(function_name) => self.run_function(&function_name, params),
					ExternFunction(function_name) => self.run_extern_function(&function_name, params),

					f => panic!("{f:?}")
				}
			}
			Tuple { items } => Value::Tuple(items.iter().map(|param| self.eval(param, frame)).collect_vec()),
			Function { is_extern, name } => if *is_extern { Value::ExternFunction(name.clone()) } else { Value::Function(name.clone()) }
		}
	}

	pub fn run_instruction(&self, instruction: &Instruction, frame: &mut StackFrame) {
		use crate::instr::InstructionKind::*;

		//println!("{instruction}");


		match instruction.kind() {
			Assign(place, value) => self.assign(place, self.eval(value, frame), frame),
			Drop(_) => todo!(),
			Eval(value) => { self.eval(value, frame); },
		}
	}

	pub fn next_after_terminator(&self, terminator: &Terminator, frame: &mut StackFrame) -> Action {
		use crate::instr::TerminatorKind::*;

		match terminator.kind() {
			Goto(basic_block_id) => Action::JumpTo(*basic_block_id),
			BranchIf { condition, positive, negative } => {
				if self.eval(condition, frame) == Value::Int(1) {
					Action::JumpTo(*positive)
				} else {
					Action::JumpTo(*negative)
				}
			}
			Switch { scrutinee, arms, default } => {
				if let Value::Int(scrutinee) = self.eval(scrutinee, frame) {
					for arm in arms.iter() {
						if scrutinee == arm.match_value {
							return Action::JumpTo(arm.arm_block);
						}
					}
	
					Action::JumpTo(*default)
				} else {
					panic!("Can only switch on an integer")
				}
			}
			ReturnVoid => Action::Return(Value::Undef),
			Return { value } => Action::Return(self.eval(&value, frame)),
			Panic => panic!("panic at src/:xx:xx"),
		}
	}

	pub fn move_value(&self, place: &Place, frame: &mut StackFrame) -> Value {
		self.copy(place, frame)
	}

	pub fn copy(&self, place: &Place, frame: &mut StackFrame) -> Value {
		self.get_ptr(place, frame).clone()
	}

	pub fn assign(&self, place: &Place, value: Value, frame: &mut StackFrame) {
		*self.get_mut_ptr(place, frame) = value;
	}

	fn get_ptr<'b>(&self, place: &Place, frame: &'b mut StackFrame) -> &'b Value {
		use crate::val::PlaceKind::*;

		match place.kind() {
			Local(local_id) => frame.get_local(*local_id).get(),
			StructField(place, field) => match self.get_ptr(place, frame) {
				Value::Struct(fields) => fields.get(field).expect("struct doesn't have field"),
				Value::Ref(value) => panic!("{:?}", unsafe { &**value }),
				v => panic!("Can't get a struct fiend of {v:?}")
			},
			CastEnumVariant(enum_place, _, _) => {
				let ptr = match self.get_mut_ptr(enum_place, frame) {
					Value::Enum(_, associated) => associated,
					_ => panic!()
				};

				if let Value::Undef = &**ptr {
					// Set pointer to the new type
					if let TypeKind::Tuple(items) = place.ty().kind() {
						*ptr.as_mut() = Value::Tuple(items.iter().map(|_| Value::Undef).collect_vec());
					} else {
						panic!("{:?}", place.ty())
					}
				}

				ptr.as_mut()
			}
			TupleItem(place, index) => {
				let value_ptr = self.get_ptr(place, frame);

				match value_ptr {
					Value::Tuple(items) => &items[*index],
					_ => panic!()
				}
			},
			Deref(rvalue) => match self.eval(rvalue, frame) {
				Value::Ref(place) => unsafe { & *place },
				_ => panic!()
			},
			Discriminant(place) => match self.get_ptr(place, frame) {
				Value::Enum(discriminant, _) => discriminant,
				_ => panic!()
			}
		}
	}

	fn get_mut_ptr<'b>(&self, place: &Place, frame: &'b mut StackFrame) -> &'b mut Value {
		use crate::val::PlaceKind::*;

		match place.kind() {
			Local(local_id) => frame.get_local_mut(*local_id).get_mut(),
			StructField(place, field) => match self.get_mut_ptr(place, frame) {
				Value::Struct(fields) => fields.get_mut(field).expect("struct doesn't have field"),
				v => panic!("Can't get a struct fiend of {v:?}")
			},
			CastEnumVariant(enum_place, _, _) => {
				let ptr = match self.get_mut_ptr(enum_place, frame) {
					Value::Enum(_, associated) => associated,
					_ => panic!()
				};

				if let Value::Undef = &**ptr {
					// Set pointer to the new type
					if let TypeKind::Tuple(items) = place.ty().kind() {
						*ptr.as_mut() = Value::Tuple(items.iter().map(|_| Value::Undef).collect_vec());
					} else {
						panic!("{:?}", place.ty())
					}
				}

				ptr.as_mut()
			}
			TupleItem(place, index) => {
				let value_ptr = self.get_mut_ptr(place, frame);

				match value_ptr {
					Value::Tuple(items) => &mut items[*index],
					t => panic!("Value {t:?} is not a tuple")
				}
			},
			Deref(rvalue) => match self.eval(rvalue, frame) {
				Value::Ref(place) => unsafe { &mut *place },
				_ => panic!()
			},
			Discriminant(place) => match self.get_mut_ptr(place, frame) {
				Value::Enum(discriminant, _) => discriminant,
				_ => panic!()
			}
		}
	}

	pub fn solo_intrinsic(&self, intrinsic: SoloIntrinsic, operand: Value) -> Value {
		use SoloIntrinsic::*;

		match operand {
			Value::Int(operand) => match intrinsic {
				INeg => Value::Int((-(operand as i64)) as u64),
				IInv => Value::Int(!operand),
				IZext16 => Value::Int(operand),
				IZext32 => Value::Int(operand),
				IZext64 => Value::Int(operand),
				ISext16 => Value::Int(operand), // todo: this is wrong
				ISext32 => Value::Int(operand),
				ISext64 => Value::Int(operand),
				ITrunc8 => Value::Int(operand & 0xff),
				ITrunc16 => Value::Int(operand & 0xffff),
				ITrunc32 => Value::Int(operand & 0xffff_ffff),
				ICnvF16 => Value::Float(operand as f64),
				ICnvF32 => Value::Float(operand as f64),
				ICnvF64 => Value::Float(operand as f64),
				ICnvF16Sig => Value::Float((operand as i64) as f64),
				ICnvF32Sig => Value::Float((operand as i64) as f64),
				ICnvF64Sig => Value::Float((operand as i64) as f64),
				AddrCnvPtr => Value::Ref(operand as *mut Value),
				_ => unreachable!()
			}

			Value::Float(operand) => match intrinsic {
				FNeg => Value::Float(-operand),
				FExt32 => Value::Float(operand),
				FExt64 => Value::Float(operand),
				FTrunc16 => Value::Float(operand),
				FTrunc32 => Value::Float(operand),
				FCnvI => Value::Int((operand as i64) as u64),
				_ => unreachable!()
			}

			Value::Ref(value) => match intrinsic {
				PtrCnvAddr => Value::Int(value as u64),
				_ => unreachable!()
			}

			_ => panic!()
		}
	}

	pub fn duo_intrinsic(&self, intrinsic: DuoIntrinsic, left: Value, right: Value) -> Value {
		use DuoIntrinsic::*;

		match (left, right) {
			(Value::Int(left), Value::Int(right)) => match intrinsic {
				IAdd => Value::Int(left + right),
				ISub => Value::Int(left - right),
				IMul => Value::Int(left * right),
				IDiv => Value::Int(left / right),
				IDivSig => Value::Int(((left as i64) / (right as i64)) as u64),
				IRem => Value::Int(left % right),
				IRemSig => Value::Int(((left as i64) % (right as i64)) as u64),
				IAnd => Value::Int(left & right),
				IOr => Value::Int(left | right),
				IXor => Value::Int(left ^ right),
				IShl => Value::Int(left << right),
				IShr => Value::Int(left >> right),
				IShrSig => Value::Int(((left as i64) << right) as u64),
				ICmpEq => if left == right { Value::Int(1) } else { Value::Int(0) },
				ICmpNeq => if left != right { Value::Int(1) } else { Value::Int(0) },
				ICmpLt => if left < right { Value::Int(1) } else { Value::Int(0) },
				ICmpLte => if left <= right { Value::Int(1) } else { Value::Int(0) },
				ICmpGt => if left > right { Value::Int(1) } else { Value::Int(0) },
				ICmpGte => if left >= right { Value::Int(1) } else { Value::Int(0) },
				ICmpLtSig => if (left as i64) < (right as i64) { Value::Int(1) } else { Value::Int(0) },
				ICmpLteSig => if (left as i64) <= (right as i64) { Value::Int(1) } else { Value::Int(0) },
				ICmpGtSig => if (left as i64) > (right as i64) { Value::Int(1) } else { Value::Int(0) },
				ICmpGteSig => if (left as i64) >= (right as i64) { Value::Int(1) } else { Value::Int(0) },
				_ => unreachable!(),
			}

			(Value::Float(left), Value::Float(right)) => match intrinsic {
				FAdd => Value::Float(left + right),
				FSub => Value::Float(left - right),
				FMul => Value::Float(left * right),
				FDiv => Value::Float(left / right),
				FRem => Value::Float(left % right),
				FCmpEq => if left == right { Value::Int(1) } else { Value::Int(0) },
				FCmpNeq => if left != right { Value::Int(1) } else { Value::Int(0) },
				FCmpLt => if left < right { Value::Int(1) } else { Value::Int(0) },
				FCmpLte => if left <= right { Value::Int(1) } else { Value::Int(0) },
				FCmpGt => if left > right { Value::Int(1) } else { Value::Int(0) },
				FCmpGte => if left >= right { Value::Int(1) } else { Value::Int(0) },
				_ => unreachable!()
			}

			(left, right) => panic!("{intrinsic:?} {left:?} {right:?}"),
		}
	}

	pub fn run_extern_function(&self, name: &str, params: Vec<Value>) -> Value {
		match name {
			"printBool" => if params[0] == Value::Int(1) { print!("true") } else { print!("false") },
			"printUInt64" => if let Value::Int(n) = params[0] {
				print!("{}", n)
			},
			"printInt64" => if let Value::Int(n) = params[0] {
				print!("{}", n as i64)
			},
			"printUInt32" => if let Value::Int(n) = params[0] {
				print!("{}", n as u32)
			},
			"printInt32" => if let Value::Int(n) = params[0] {
				print!("{}", n as i32)
			},
			"printUInt16" => if let Value::Int(n) = params[0] {
				print!("{}", n as u16)
			},
			"printInt16" => if let Value::Int(n) = params[0] {
				print!("{}", n as i16)
			},
			"printUInt8" => if let Value::Int(n) = params[0] {
				print!("{}", n as u8)
			},
			"printInt8" => if let Value::Int(n) = params[0] {
				print!("{}", n as i8)
			},
			"printChar" => if let Value::Int(n) = params[0] {
				print!("{}", char::from_u32(n as u32).unwrap())
			}
			"printFloat" => if let Value::Float(n) = params[0] {
				print!("{}", n as f32)
			}
			"printDouble" => if let Value::Float(n) = params[0] {
				print!("{}", n)
			}
			"printString" => if let (Value::Ref(ptr), Value::Int(len)) = (&params[0], &params[1]) {
				for i in 0..*len {
					if let Value::Int(c) = unsafe { &*ptr.add(i as usize) } {
						print!("{}", *c as u8 as char)
					}
				}
			}

			"printLine" => println!(),

			_ => panic!("Function {name} isn't defined")
		}

		Value::Tuple(vec![])
	}
}

pub enum Action {
	JumpTo(BasicBlockId),
	Return(Value),
}

/*
left to do:
- Locals
- Documentation
- More structs
- Value building
- Place building
- Enums
- Execution Engine
*/

/*

Execution Engine:

StackFrame:
	locals [Var]

Var:
	val Val
	ty Type

Val:
	Integer(u64)
	Float(f64)
	String(String)
	Tuple(Vec<Val>)
	StructInstance(HashMap<String, Val>)
	EnumInstance(String, Val)


 */