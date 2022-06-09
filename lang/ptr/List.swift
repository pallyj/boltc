let initial_capacity = 2;

public struct List<T> {
	private var ptr: UnsafeMutablePointer<T>
	private var count: UInt
	private var capacity: UInt

	public init() = init(capacity initial_capacity)

	public init(capacity capacity: UInt) {
		self.len = 0;
		self.capacity = capacity;
		self.ptr = allocate(self.capacity).cast();
	}

	public init(repeating value: T, count count: UInt) {
		init(nextPowerOf2(count))

		while i < count {
			(ptr + i).set(value)

			i += 1;
		}
	}

	private mutating func resize() {
		let newCapacity = nextPowerOf2(count).max(2)

		if newCapacity > capacity {
			let var i = 0;

			let newPtr = allocate(self.capacity);

			while i < count {
				(ptr + i).set((newPtr + i).get())

				i += 1;
			}

			self.capacity = newCapacity;
			self.ptr = newPtr;
		}
	}
}

private func nextPowerOf2(n: UInt) -> UInt {
	let var power = 0;

	while n > 0 {
		n >>= 1;
		power += 1;
	}

	1 << power;
}

func allocate(n_bytes: UInt) -> UnsafePointer<()>

List(repeating: 0, count: 10)