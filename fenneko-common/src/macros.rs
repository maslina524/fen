#[macro_export]
macro_rules! println {
    () => {{
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_char(&mut std, '\n');
    }};
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stdout::new();
        let args = ::core::format_args!($($tt)+);
        let _ = ::core::fmt::Write::write_fmt(&mut std, args);
        let _ = ::core::fmt::Write::write_char(&mut std, '\n');
    }};
}

#[macro_export]
macro_rules! print {
    () => {{}};
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stdout::new();
        let args = ::core::format_args!($($tt)+);
        let _ = ::core::fmt::Write::write_fmt(&mut std, args);
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_char(&mut std, '\n');
    }};
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stderr::new();
        let args = ::core::format_args!($($tt)+);
        let _ = ::core::fmt::Write::write_fmt(&mut std, args);
        let _ = ::core::fmt::Write::write_char(&mut std, '\n');
    }};
}

#[macro_export]
macro_rules! eprint {
    () => {{}};
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stderr::new();
        let args = ::core::format_args!($($tt)+);
        let _ = ::core::fmt::Write::write_fmt(&mut std, args);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn println_test() {
        crate::print!("I love {}", "Rust");
        crate::println!();
        crate::println!("We love {}", "Rust");
    }
}