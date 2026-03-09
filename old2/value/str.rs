pub struct DoughStr {
    chars: String
}

impl DoughStr {
    pub fn new(s: impl Into<String>) -> Self {
        Self { chars: s.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.chars
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}