#[test]
fn test_type_to_string() {
    assert_eq!(Type::Void.to_string(), "void");
    assert_eq!(Type::Integer { bits: 32 }.to_string(), "i32");
    assert_eq!(Type::Float { bits: 32 }.to_string(), "f32");
    assert_eq!(Type::Integer { bits: 32 }.func_type(vec![Type::Integer { bits: 32 }, Type::Integer { bits: 32 }])
                                         .to_string(),
               "(i32, i32): i32")
}

#[test]
fn test_function() {
    let mut builder = Builder::new();

    let func = Function::new("helloWorld", Type::Integer { bits: 32 }.func_type(vec![]));

    let block = func.append_block("start");

    builder.position_at_end(&block);

    let a = builder.build_integer_literal(32, 1);
    let b = builder.build_integer_literal(32, 5);

    let c = builder.build_binary_intrinsic(BinaryIntrinsicFn::IntegerAdd, a, b);

    builder.build_return(Some(c));

    println!("{func}");
}

// func testAdd ( ) {
//
// %0 = integer_literal 0x0 : i64
// %1 = integer_literal 0x64 : i64
// %2 = intrinsic "integer64Add" ( i64 %0, i64 %1 ) : i64
// return (i64 %2)
//
// }
//
// func testMul ( %a : i64, %b: i64 ) : i64 {
//
// %0 = integer_literal 0x1 : i64 // 1
// %1 = intrinsic "integer64Sub" ( i64 %a, i64 %0 ) : i64 // (a - 1)
// %2 = function "testMul" : ( i64, i64 ): i64 // testMul
// %3 = call %2 ( %1, %b ) : i64 // testMul(a - 1, b)
// %4 = intrinsic "integer64Add" ( i64 %3, i64 %b ) : i64 // testMul(a - 1, b) + b
//
// return (i64 %4)
//
// }
//

use crate::{code::Function, typ::Type, value::BinaryIntrinsicFn, Builder};
