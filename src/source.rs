#[derive(Debug, Clone, Copy)]
pub struct Source<'a> {
    source: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn len(&self) -> usize {
        self.source.len()
    }

    pub fn char_at(&self, byte_index: usize) -> Option<char> {
        self.source.get(byte_index..)?.chars().next()
    }

    pub fn slice(&self, start: usize, end: usize) -> Option<&'a str> {
        self.source.get(start..end)
    }
}
