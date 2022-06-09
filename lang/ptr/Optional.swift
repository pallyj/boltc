public enum Optional<T> {
	case none
	case some(T)

	public func map(f: func(T) -> U) -> Optional<U> {
		match self {
			.none => .none,
			.some(x) => .some(f(x))
		}
	}

	public func flatMap(f: func(T) -> Optional<U>) -> Optional<U> {
		match self {
			.none => .none,
			.some(x) => f(x)
		}
	}

	public func unwrap() -> T {
		match self {
			.some(x) => x,
			.none => panic()
		}
	}

	public operator func coalesce(or_else: T) -> T {
		match self {
			.some(x) => x,
			.none => or_else
		}
	}
}

