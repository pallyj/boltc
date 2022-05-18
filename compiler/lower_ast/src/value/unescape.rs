pub fn unescape(text: &str) -> Result<String, Vec<(String, usize)>> {
	let mut accumulator = String::with_capacity(text.len());
	let mut is_escaped = false;
	let mut char_iter = text.chars();

	let mut errors = Vec::new();
	let mut idx = 0;

	while let Some(c) = char_iter.next() {
		idx += 1;
		if !is_escaped {
			if c == '\\' {
				is_escaped = true;
			} else {
				accumulator.push(c);
			}

			continue
		}

		match c {
			'n' => accumulator.push('\n'),
			'r' => accumulator.push('\r'),
			'b' => accumulator.push('\x08'),
			'f' => accumulator.push('\x0c'),
			't' => accumulator.push('\t'),
			'v' => accumulator.push('\x0b'),
			'u' => 
				match read_unicode_escape(&mut char_iter) {
					Ok(c) => accumulator.push(c),
					Err(err) => errors.push((err, idx))
				}
			'0' => accumulator.push('\0'),
			'\"' => accumulator.push('\"'),
			'\'' => accumulator.push('\''),
			'\\' => accumulator.push('\\'),
			c => errors.push((format!("error: invalid escape sequence \\{c}"), idx)),
		}

		is_escaped = false;
	}

	if errors.is_empty() {
		Ok(accumulator)
	} else {
		Err(errors)
	}
}

fn read_unicode_escape(iterator: &mut impl Iterator<Item = char>) -> Result<char, String> {
	if iterator.next() != Some('(') {
		return Err("expected ( in unicode escape sequence".to_string())
	}

	let mut i = 0;
	let mut digit_code = 0;

	loop {
		if i >= 6 {
			return Err("unicode escape code must be less than 5 hex digits".to_string())
		}

		match iterator.next() {
			Some(')') => {break}
			Some(c) => {
				digit_code *= 0x10;
				if let Some(digit) = c.to_digit(16) {
					digit_code += digit;
				} else {
					return Err(format!("{c} is not a valid hexadecimal digit"))
				}
			}
			None => { return Err("unicode escape ended in the middle of code".to_string()) }
		}

		i += 1;
	}

	if let Some(c) = char::from_u32(digit_code) {
		Ok(c)
	} else {
		Err(format!("\\u({digit_code:x}) is not a valid escape sequence"))
	}
}