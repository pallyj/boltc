use blir::{value::ValueKind, typ::TypeKind, pattern::PatternKind};

use crate::{matrix::{PatternMatrix}, solver::Maranget};

/*
	match (a, b)
	| (_, 1) => print("right 1")
	| (1, _) => print("left 1")
	| x => print(x.item1)

1
	_ => $0
_
	1 => $1
	_ => $2


(a, b)

(_, 1)
(1, _)
x

a b

_ 1 => $1
1 _ => $2
_ _ => $3 |-> binds x
*/
// Status:
// The matrix is expanded correctly, values are bound right
#[test]
fn test_basic_example() {
	let integer_type = TypeKind::Integer { bits: 64 }.anon();
	let tuple_type = TypeKind::Tuple(vec![
		integer_type.clone(),
		integer_type.clone(),
	]).anon(); 

	let match_value = ValueKind::Tuple(vec![
		ValueKind::LocalVariable("a".to_string()).anon(integer_type.clone()),
		ValueKind::LocalVariable("b".to_string()).anon(integer_type.clone()),
	]).anon(tuple_type.clone());

	let pat1 = PatternKind::Tuple { items: vec![
		PatternKind::Wildcard.with_type(integer_type.clone()),
		PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
	] }.with_type(tuple_type.clone());
	let pat2 = PatternKind::Tuple { items: vec![
		PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
		PatternKind::Wildcard.with_type(integer_type.clone()),
	] }.with_type(tuple_type.clone());
	let pat3 = PatternKind::Bind("x".to_string()).with_type(tuple_type.clone());

	let matrix = PatternMatrix::construct(
		match_value,
		vec![pat1, pat2, pat3]);

	let matrix = matrix.expand();

	println!("{:?}", matrix.solve::<Maranget>());
}
/*

match ((a, b), (c, d))
| ((1, n), m) => {}
| (m, (1, n)) => {}
| ((m, n), (1, 2)) => {}
| ((1, 2), (n, m)) => {}
| (n, m) => {}


*/

#[test]
fn test_complex_example() {
	let integer_type = TypeKind::Integer { bits: 64 }.anon();
	let tuple_type = TypeKind::Tuple(vec![
		integer_type.clone(),
		integer_type.clone(),
	]).anon(); 
	let super_tuple_type = TypeKind::Tuple(vec![
		tuple_type.clone(),
		tuple_type.clone(),
	]).anon(); 

	let match_value = ValueKind::LocalVariable("a".to_string()).anon(super_tuple_type.clone());

	let pat1 = PatternKind::Tuple { items: vec![
		PatternKind::Tuple { items: vec![
			PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
			PatternKind::Bind("n".to_string()).with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone()),
		PatternKind::Bind("m".to_string()).with_type(tuple_type.clone()),
	] }.with_type(super_tuple_type.clone());
	let pat2 = PatternKind::Tuple { items: vec![
		PatternKind::Bind("m".to_string()).with_type(tuple_type.clone()),
		PatternKind::Tuple { items: vec![
			PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
			PatternKind::Bind("n".to_string()).with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone())
	] }.with_type(super_tuple_type.clone());
	// ((1, 2), (n, m)) => {}
	// (n, m) => {}
	let pat3 = PatternKind::Tuple { items: vec![
		PatternKind::Tuple { items: vec![
			PatternKind::Bind("n".to_string()).with_type(integer_type.clone()),
			PatternKind::Bind("m".to_string()).with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone()),
		PatternKind::Tuple { items: vec![
			PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
			PatternKind::Literal { value: ValueKind::IntLiteral(2).anon(integer_type.clone()) }.with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone()),
	] }.with_type(super_tuple_type.clone());
	let pat4 = PatternKind::Tuple { items: vec![
		PatternKind::Tuple { items: vec![
			PatternKind::Literal { value: ValueKind::IntLiteral(1).anon(integer_type.clone()) }.with_type(integer_type.clone()),
			PatternKind::Literal { value: ValueKind::IntLiteral(2).anon(integer_type.clone()) }.with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone()),
		PatternKind::Tuple { items: vec![
			PatternKind::Bind("n".to_string()).with_type(integer_type.clone()),
			PatternKind::Bind("m".to_string()).with_type(integer_type.clone()),
		] }.with_type(tuple_type.clone()),
	] }.with_type(super_tuple_type.clone());
	let pat5 = PatternKind::Tuple { items: vec![
		PatternKind::Bind("n".to_string()).with_type(tuple_type.clone()),
		PatternKind::Bind("m".to_string()).with_type(tuple_type.clone()),
	] }.with_type(super_tuple_type.clone());

	let matrix = PatternMatrix::construct(
		match_value,
		vec![pat1, pat2, pat3, pat4, pat5]);

	let matrix = matrix.expand();

	println!("{:?}", matrix.solve::<Maranget>());
}