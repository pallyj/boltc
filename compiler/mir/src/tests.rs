use errors::Span;

use crate::instr::Terminator;
use crate::{Project, Builder};
use crate::ty::Type;
use crate::val::{RValue, DuoIntrinsic};


macro_rules! func {
	($builder:ident ; $name:ident ($($params:expr),*) -> ($return_type:expr) {
		$(let var $var_name:ident: $var_type:expr;)* in

		$($bb_name:ident: {
			$($bb_code:stmt)*
		})*
	}) => {
		let function = $builder.add_function(stringify!($name), vec![ $($params),* ], $return_type);
		$builder.position_on_func(function);

		$(let $var_name = $builder.build_local($var_type);)*

		$(
			let $bb_name = $builder.append_block();
		)*

		$(
			$builder.position_at_end($bb_name);

			$($bb_code)*
		)*
	};
}

#[test]
fn type_to_string() {
	/*assert_eq!("i1", format!("{}", Type::int(1)));
	assert_eq!("i8", format!("{}", Type::int(8)));
	assert_eq!("i16", format!("{}", Type::int(16)));
	assert_eq!("i32", format!("{}", Type::int(32)));
	assert_eq!("i64", format!("{}", Type::int(64)));

	assert_eq!("f16", format!("{}", Type::float(16)));
	assert_eq!("f32", format!("{}", Type::float(32)));
	assert_eq!("f64", format!("{}", Type::float(64)));

	assert_eq!("[i64; 5]", format!("{}", Type::int(64).array(5)));
	assert_eq!("[&i64; 5]", format!("{}", Type::int(64).pointer().array(5)));
	assert_eq!("&[i64; 5]", format!("{}", Type::int(64).array(5).pointer()));

	assert_eq!("(i64, i32)", format!("{}", Type::tuple(vec![ Type::int(64), Type::int(32) ])));
	assert_eq!("()", format!("{}", Type::tuple(vec![ ])));

	assert_eq!("func (i64) -> i64", format!("{}", Type::int(64).func(vec![ Type::int(64) ])));

	assert_eq!("func (&[i64; 10]) -> i64", format!("{}", Type::int(64).func( vec![ Type::int(64).array(10).pointer() ] )));*/
}

#[test]
fn val_to_string() {
	assert_eq!("const 42", format!("{}", RValue::const_int(42, Type::int(32), Span::empty()) ));
	assert_eq!("const 3.14", format!("{}", RValue::const_float(3.14, Type::float(32), Span::empty()) ));
	assert_eq!("const \"Hello, World!\"", format!("{}", RValue::const_string("Hello, World!", Type::int(8).shared_pointer(), Span::empty()) ));


}

#[test]
fn functions_and_blocks() {
	/*let expected =
r#"func meaningOfLife (i64) -> i64 { 
	bb0 : {
		return const 42
	}
}"#;*/

	let mut project = Project::new("main");
	let mut builder = Builder::new(&mut project);

	let meaning_of_life = builder.add_function("meaningOfLife", vec![ Type::int(64) ], Type::int(64));
	builder.position_on_func(meaning_of_life);

	let bb0 = builder.append_block();
	builder.position_at_end(bb0);

	builder.build_terminator(Terminator::returns(RValue::const_int(42, Type::int(64))));

	println!("{project}");
	//assert_eq!(format!("{project}"), expected);
}

#[test]
fn loop_to_ten() {
	let mut project = Project::new("loop");
	let mut builder = Builder::new(&mut project);

	let loop_func = builder.add_function("loopToTen", vec![ ], Type::int(64));
	builder.position_on_func(loop_func);

	let bb0 = builder.append_block();
	let bb1 = builder.append_block();
	let bb2 = builder.append_block();

	let acc = builder.build_local(Type::int(64));
	let added = builder.build_local(Type::int(64));
	let keep_going = builder.build_local(Type::int(1));

	builder.position_at_end(bb0);

	builder.build_assign(&acc, RValue::const_int(0, Type::int(64)));
	builder.build_terminator(Terminator::goto(bb1));

	builder.position_at_end(bb1);

	// Increment acc by 1
	builder.build_assign(&added, RValue::intrinsic2(DuoIntrinsic::IAdd, acc.copy(), RValue::const_int(1, Type::int(64))));
	builder.build_assign(&acc, added.copy());

	// Check if acc is bigger than 10
	builder.build_assign(&keep_going, RValue::intrinsic2(DuoIntrinsic::ICmpLt, acc.copy(), RValue::const_int(10, Type::int(64))));

	// Check if the accumulator has reached its value
	builder.build_terminator(Terminator::branch_if(keep_going.copy(), bb1, bb2));

	builder.position_at_end(bb2);

	builder.build_terminator(Terminator::returns(acc.copy()));

	//println!("{project}");

	let ten = project.execute().run_function("loopToTen", vec![]);

	println!("{ten:?}")
}

#[test]
fn construct_tuple() {
	let mut project = Project::new("loop");
	let mut builder = Builder::new(&mut project);

	let get_tuple = builder.add_function("getTuple", vec![], Type::tuple(vec![Type::int(64), Type::int(64)]));
	{
		builder.position_on_func(get_tuple);

		let bb0 = builder.append_block();
		builder.position_at_end(bb0);

		let tuple = builder.build_local(Type::tuple(vec![Type::int(64), Type::int(64)]));

		builder.build_assign(&tuple, RValue::tuple(vec![ RValue::const_int(100, Type::int(64)), RValue::const_int(200, Type::int(64)) ]));

		builder.build_terminator(Terminator::returns(tuple.copy()))
	}

	let destructure_tuple = builder.add_function("destructureTuple", vec![], Type::int(64));
	{
		builder.position_on_func(destructure_tuple);

		let tup = builder.build_local(Type::tuple(vec![Type::int(64), Type::int(64)]));

		let bb0 = builder.append_block();
		builder.position_at_end(bb0);

		let func = builder.build_function("getTuple");

		builder.build_assign(&tup, func.call(vec![]));

		builder.build_terminator(Terminator::returns(tup.tuple_item(0).copy()))
	}

	println!("{project}");

	println!("{:?}", project.execute().run_function("destructureTuple", vec![]));
}

#[test]
fn macros() {
	let mut project = Project::new("macros");
	let mut builder = Builder::new(&mut project);

	func! { builder; hello_world( ) -> (Type::void()) {
		let var acc: Type::int(64);
		let var added: Type::int(64);
		let var keep_going: Type::int(1); in

		bb0: {
			builder.build_assign(&acc, RValue::const_int(0, Type::int(64)))
			builder.build_terminator(Terminator::goto(bb1))
		}

		bb1: {
			// Increment acc by 1
			builder.build_assign(&added, RValue::intrinsic2(DuoIntrinsic::IAdd, acc.copy(), RValue::const_int(1, Type::int(64))))
			builder.build_assign(&acc, added.copy())

			// Check if acc is bigger than 10
			builder.build_assign(&keep_going, RValue::intrinsic2(DuoIntrinsic::ICmpLt, acc.copy(), RValue::const_int(100, Type::int(64))))

			// Check if the accumulator has reached its value
			builder.build_terminator(Terminator::branch_if(keep_going.copy(), bb1, bb2))
		}

		bb2: {
			builder.build_terminator(Terminator::returns(acc.copy()))
		}
	}};



	println!("{:?}", project.execute().run_function("hello_world", vec![]));
}

#[test]
fn int_struct() {
	let mut project = Project::new("int_struct");
	let mut builder = Builder::new(&mut project);

	let int = builder.add_struct("Int", true, false);

	builder.add_struct_fields("Int", vec![(String::from("repr"), Type::int(64))]);

	func! { builder; hello_world() -> (Type::void()) {
		let var acc: int.ty();
		let var added: int.ty();
		let var keep_going: Type::int(1); in

		bb0: {
			let repr_field = builder.build_field(&acc, "repr")
			builder.build_assign(&repr_field, RValue::const_int(0, Type::int(64)))
			builder.build_terminator(Terminator::goto(bb1))
		}

		bb1: {
			let added_field = builder.build_field(&added, "repr")
			let acc_field = builder.build_field(&acc, "repr")

			// Increment acc by 1
			builder.build_assign(&added_field, RValue::intrinsic2(DuoIntrinsic::IAdd, acc_field.copy(), RValue::const_int(1, Type::int(64))))
			builder.build_assign(&acc, added.copy())

			// Check if acc is bigger than 10
			builder.build_assign(&keep_going, RValue::intrinsic2(DuoIntrinsic::ICmpLt, acc_field.copy(), RValue::const_int(100, Type::int(64))))

			// Check if the accumulator has reached its value
			builder.build_terminator(Terminator::branch_if(keep_going.copy(), bb1, bb2))
		}

		bb2: {
			builder.build_terminator(Terminator::return_void())
		}
	}};

	println!("{:?}", project.execute().run_function("hello_world", vec![]))
}