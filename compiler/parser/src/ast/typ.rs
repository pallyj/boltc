/*

Type syntax 0.3

Named: *ident*
Member: **type** `.` *ident*
Unit: `(` `)`
Function: `func` `(` (**type**),* `)` `:` **type**
Infer: `_`

*/

pub enum Type {
	Named(String),
	Member(Box<Type>, String),
	Unit,
	Function(Vec<Type>, Box<Type>)
}