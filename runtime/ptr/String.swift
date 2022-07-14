struct StringSlice {
	private var ptr: UnsafePointer<UInt8>,
	private var len: UInt,

	private init(ptr: RawPointer<UInt8>, len: i64) {
		self.ptr = UnsafePointer(repr: ptr)
		self.len = UInt(repr: len)
	}

	public var length {
		get = self.len
	}

	public struct Utf8View {
		private var slice: StringSlice

		public mutating func next() -> Optional<UInt8> {
			if self.slice.len <= 0 {
				return .none
			}

			self.slice.len -= 1;
			self.slice.ptr += 1;

			.some(self.slice.ptr.get())
		}
	}

	public func utf8() -> Utf8View {
		Utf8(slice: self)
	}

	public struct CharView {
		private var utf8: Utf8View

		public mutating func next() -> Optional<Char> {
			let first = try utf8.next();

			// A 1-byte character is most common, check for it first
			if first < 0x80 {
				return .some(Char(UInt32(first)))
			}

			let mut extraByteAccumulation = 0;
			let mut numberOfExtraBytes = 1;

			extraByteAccumulation |= UInt32(try utf8.next())

			if first >= 0xE0 {
				extraByteAccumulation <<= 6;
				extraByteAccumulation |= UInt32(try utf8.next())
				numberOfExtraBytes += 1;

				if first >= 0xF0 {
					extraByteAccumulation <<= 6;
					extraByteAccumulation |= UInt32(match utf8.next())
					numberOfExtraBytes += 1;
				}
			}

			let firstByteContribution = (first & (0x7f >> numberOfExtraBytes)) << (6 * numberOfExtraBytes);

			return Char(firstByteContribution | extraByteAccumulation)
		}
	}

	public func chars() -> CharView {
		CharView(slice: self.utf8())
	}
}