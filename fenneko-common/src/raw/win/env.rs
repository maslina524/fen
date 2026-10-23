use alloc::{string::String, sync::Arc, vec::Vec};

use winapi::api::process::GetCommandLine;

pub mod consts {
    pub const OS: &str = "windows";
    pub const MAX_PATH_LEN: usize = 32_767;
}

pub struct ArgsInner {
    source: String,
    chars: Vec<char>,
    pos: usize,
}

impl ArgsInner {
    #[must_use]
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Self {
            source,
            chars,
            pos: 0,
        }
    }

    fn peek(&self) -> char {
        self.chars[self.pos]
    }

    fn pop(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    const fn in_bound(&self) -> bool {
        self.pos < self.chars.len()
    }

    fn build_str(&self, start: usize, len: usize) -> Arc<str> {
        let string = &self.source[start..start + len];
        Arc::from(string)
    }

    fn skip_whitespaces(&mut self) {
        while self.in_bound() && self.peek().is_whitespace() {
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

    fn read_just_text_len(&mut self) -> usize {
        let start = self.pos;

        while self.in_bound() && !self.peek().is_whitespace() {
            self.pos += 1;
        }

        self.pos - start
    }
}

impl Iterator for ArgsInner {
    type Item = Arc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.in_bound() {
            return None;
        }

        self.skip_whitespaces();
        if !self.in_bound() {
            return None;
        }

        if self.peek() == '\"' {
            let start = self.pos + 1;
            let len = self.read_in_quotes_len();
            return Some(self.build_str(start, len));
        }

        let start = self.pos;
        let len = self.read_just_text_len();
        Some(self.build_str(start, len))
    }
}

#[must_use]
pub fn args_string() -> String {
    GetCommandLine()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! next_is {
        ($args:expr, $lit:literal) => {
            assert_eq!(Some($lit), $args.next().as_deref());
        };
    }

    #[test]
    fn args_check_quotes_test() {
        let mut args = ArgsInner::new("\"Hello\" \"World\" \"\"".to_owned());
        next_is!(args, "Hello");
        next_is!(args, "World");
        next_is!(args, "");
    }

    #[test]
    fn args_check_all_test() {
        let mut args = ArgsInner::new("git commit -m \"Initial commit\"".to_owned());
        next_is!(args, "git");
        next_is!(args, "commit");
        next_is!(args, "-m");
        next_is!(args, "Initial commit");
    }

    #[test]
    fn args_splash_test() {
        let mut args = ArgsInner::new("git commit -m \"Initial commit\"".to_owned());
        next_is!(args, "git");
        next_is!(args, "commit");
        next_is!(args, "-m");
        next_is!(args, "Initial commit");
    }
}
