import intrinsics

public struct UnsafePointer<T> {
	private var repr: RawPointer<T>

	///
	/// Initializes an `UnsafePointer<T>` from a raw address
	///
	public init(address addr: UInt) {
		self.repr = rawPointerFromAddr(addr.repr)
	}

	///
	/// Initializaes an `UnsafePointer<T>` referring to a field
	///
	public init(repr: shared T) {
		self.repr = rawPointerRef(shared T)
	}

	///
	/// Adds an index from the `UnsafePointer<T>`
	/// The offset is in terms of T, not bytes
	///
	public operator func add(n: Int) -> Self {
		Self(repr: rawPointerAdd(self.repr, n.repr))
	}

	///
	/// Subtracts an index from the `UnsafePointer<T>`
	/// The offset is in terms of T, not bytes
	///
	public operator func sub(m: Int) -> Self {
		Self(repr: rawPointerAdd(self.repr, (-n).repr))
	}

	///
	/// Copies the value out of an UnsafePointer<T>
	///
	public func get() -> T {
		rawPointerDeref(self.repr)
	}
}