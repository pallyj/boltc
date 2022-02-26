// rust

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token {

// Variant(params),
//   String => String
//   Int => i64
//   UInt => u64

// Eof,

//}

// impl GenericToken for Token {
// 	   fn eof() -> Self {
//  	  Self::Eof,
//     }
// }

//Symbol Table

/*
^0 := type_resolver();

%0 = create_on_stack<^0>;

%1 = get_operator_impl<^0>("neg");

%2 = call_fn %1 (%0);


/*
BLIR

allocation:
# Creates an object with an unknown type
create_some

# Creates an object on the stack
create_on_stack

# Creates a reference counted object
create_object

# Boxes a struct
create_boxed

static dispatch:
get_operator_impl
get_protocol_func
get_object_func
get_static_func
get_member_value
set_member_value

objects:
retain_strong
release_strong
retain_weak
release_weak
get_pointer

control flow:

# Implements if and match
switch

# Raw loop
loop

# Leaves a loop
break

# Continues a loop
continue




*/





*/

/*TypeInference

.equals
.assignsFrom
.implements
.subTypeOf
.hasMethod
.hasProperty
.passTo*/