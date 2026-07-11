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
        '[' => false,
        c => {
            if c == t[ti] {
                glob_rec(p, pi + 1, t, ti + 1)
            } else {
                false
            }
        }
    }
}