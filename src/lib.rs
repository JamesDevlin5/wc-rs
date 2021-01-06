pub struct Count {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    line_len: usize,
}

impl Count {
    pub fn new() -> Self {
        Count {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
            line_len: 0,
        }
    }
}
