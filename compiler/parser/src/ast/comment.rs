use std::fmt::Display;

ast!(struct Comment(Docs));

impl Comment {
	pub fn docs(&self) -> String {
		self.0.children()
			.map(|comment| format!("{}\n", comment.text()))
			.collect::<String>()
	}
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.docs())
    }
}