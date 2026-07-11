use alloc::vec::Vec;

pub fn glob(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_rec(&p, 0, &t, 0)
}

fn glob_rec(p: &[char], pi: usize, t: &[char], ti: usize) -> bool {
    if pi == p.len() {
        return ti == t.len();
    }

    if ti == t.len() {
        return p[pi..].iter().all(|&c| c == '*');
    }

    match p[pi] {
        '?' => glob_rec(p, pi + 1, t, ti + 1),
        '*' => {
            for i in ti..=t.len() {
                if glob_rec(p, pi + 1, t, i) {
                    return true;
                }
            }
            false
        }
        '[' => {
            let (matched, new_pi) = parse_class(p, pi, t[ti]);
            if matched {
                glob_rec(p, new_pi, t, ti + 1)
            } else {
                false
            }
        }
        c => {
            if c == t[ti] {
                glob_rec(p, pi + 1, t, ti + 1)
            } else {
                false
            }
        }
    }
}

fn parse_class(p: &[char], pi: usize, ch: char) -> (bool, usize) {
    if pi + 1 >= p.len() {
        return (false, pi);
    }

    let mut idx = pi + 1;
    let mut negate = false;

    if p[idx] == '!' || p[idx] == '^' {
        negate = true;
        idx += 1;
        if idx >= p.len() || p[idx] == ']' {
            return (false, pi);
        }
    }

    let start_idx = idx;
    let mut matched = false;

    while idx < p.len() && p[idx] != ']' {
        if p[idx] == '-'
            && idx > start_idx
            && idx + 1 < p.len()
            && p[idx + 1] != ']'
        {
            let start = p[idx - 1];
            let end = p[idx + 1];
            if start <= ch && ch <= end {
                matched = true;
            }
            idx += 2;
            continue;
        }

        if p[idx] == ch {
            matched = true;
        }
        idx += 1;
    }

    if idx < p.len() && p[idx] == ']' {
        let new_pi = idx + 1;
        if negate {
            matched = !matched;
        }
        return (matched, new_pi);
    }

    (false, pi)
}