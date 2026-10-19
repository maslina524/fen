use alloc::vec::Vec;

pub struct ArgsInner<'arg> {
    source: &'arg str,
    chars: Vec<char>,
    pos: usize
}

impl<'arg> ArgsInner<'arg> {
    pub fn new(source: &'arg str) -> Self {
        let chars = source.chars().collect();
        Self { source, chars, pos: 0 }
    }

    fn peek(&self) -> char {
        self.chars[self.pos]
    }

    fn pop(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn build_str(&self, start: usize, len: usize) -> &'arg str {
        &self.source[start..start + len]
    }

    fn skip_whitespaces(&mut self) {
        while self.pos < self.chars.len() && self.peek().is_whitespace() {
            self.pos += 1;
        }
    }

    fn read_in_quotes_len(&mut self) -> usize {
        self.pop(); // "
        let start = self.pos;

        loop {
            if self.peek() == '\"' {
                break;
            }

            self.pos += 1;
        }

        let len = self.pos - start;
        self.pop(); // "
        len
    }
}

impl<'arg> Iterator for ArgsInner<'arg> {
    type Item = &'arg str;
    fn next(&mut self) -> Option<Self::Item> {
        let source_len = self.chars.len();
        if self.pos >= source_len {
            return None;
        }

        while self.pos < source_len {
            self.skip_whitespaces();

            if self.peek() == '\"' {
                let start = self.pos + 1;
                let len = self.read_in_quotes_len();
                return Some(self.build_str(start, len))
            }

            self.pos += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_check_quotes() {
        let mut args = ArgsInner::new("\"Hello\" \"World\" ");
        assert_eq!(Some("Hello"), args.next());
        assert_eq!(Some("World"), args.next());
    }
}