pub trait CharExts {
	fn is_newline(self) -> bool;
	fn is_whitespace(self) -> bool;

	fn is_binary_digit(self) -> bool;
	fn is_octal_digit(self) -> bool;
	fn is_hex_digit(self) -> bool;
	fn is_decimal_digit(self) -> bool;
}

impl CharExts for char {
	fn is_newline(self) -> bool {
		self == '\r' || self == '\n'
	}

	fn is_whitespace(self) -> bool {
		self == ' ' ||
		self == '\0' ||
		self == '\x09' ||
		self == '\x0A' ||
		self == '\x0B' ||
		self == '\x0C' ||
		self == '\x0D'
	}


	fn is_binary_digit(self) -> bool {
		self == '0' || self == '1'
	}

	fn is_octal_digit(self) -> bool {
		self >= '0' && self < '8'
	}

	fn is_decimal_digit(self) -> bool {
		self >= '0' && self <= '9'
	}

	fn is_hex_digit(self) -> bool {
		(self >= '0' && self <= '9') ||
		(self >= 'a' && self <= 'f') ||
		(self >= 'A' && self <= 'F')
	}
}