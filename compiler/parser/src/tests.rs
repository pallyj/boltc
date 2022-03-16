use crate::{parser::Parser, ast::{Root}};


#[test]
fn test_code() {
	let mut parser = Parser::new(r#"
	import intrinsics

public struct Int64 {
	var repr: i64
	public struct Int32 {}

	public static func max(): Int64 {
		return Self(0x7fffffffffffffff)
	}

	public static func min(): Int64 {
		return Self(0x8000000000000000)
	}

	public func add(b: Self): Self {
		return Self(integer64Add(self.repr, b.repr))
	}

	public func sub(b: Self): Self {
		return Self(integer64Sub(self.repr, b.repr))
	}

	public func mul(b: Self): Self {
		return Self(integer64Mul(self.repr, b.repr))
	}

	public func over(b: Self): Self {
		return Self(integer64DivSig(self.repr, b.repr))
	}

	public func mod(b: Self): Self {
		return Self(integer64RemSig(self.repr, b.repr))
	}


	public func xor(b: Self): Self {
		return Self(integer64Xor(self.repr, b.repr))
	}

	public func and(b: Self): Self {
		return Self(integer64And(self.repr, b.repr))
	}

	public func or(b: Self): Self {
		return Self(integer64Or(self.repr, b.repr))
	}

	public func shl(b: Self): Self {
		return Self(integer64Shl(self.repr, b.repr))
	}

	public func shr(b: Self): Self {
		return Self(integer64ShrSig(self.repr, b.repr))
	}


	public func eq(b: Self): i1 {
		return integer64CmpEq(self.repr, b.repr)
	}

	public func neq(b: Self): i1 {
		return integer64CmpNeq(self.repr, b.repr)
	}

	public func gt(b: Self): i1 {
		return integer64CmpGtSig(self.repr, b.repr)
	}

	public func gte(b: Self): i1 {
		return integer64CmpGteSig(self.repr, b.repr)
	}

	public func lt(b: Self): i1 {
		return integer64CmpLtSig(self.repr, b.repr)
	}

	public func lte(b: Self): i1 {
		return integer64CmpLteSig(self.repr, b.repr)
	}


	public func truncate32(): i32 {
		return integer64Trunc32(self.repr)
	}

	public func truncate16(): i16 {
		return integer64Trunc16(self.repr)
	}

	public func truncate8(): i8 {
		return integer64Trunc8(self.repr)
	}

	public func negate(): Self {
		return Self(integer64Negate(self.repr))
	}
}

func main(): i64 {
	printi(gcd(21, 9));

	printi(gcd(100, 65));

	return 0
}

func gcd(a: Int64, b: Int64): Int64 {
	if a.lt(b) {
		return gcd(b, a)
	}

	if a.mod(b).eq(0) {
		return b
	}

	return gcd(b, a.mod(b))
}
	"#);

	parser.operator_factory().register_intrinsics();

	let block = parser.parse_file();

	println!("{:?}", block);

	let block = Root::cast(block.root).unwrap();
	println!("{block:?}");
}

// Static
// Imports