static CHARSET: &[(&str, char)] = &[
    ("top_left", '┌'),
    ("top_right", '┐'),
    ("bottom_left", '└'),
    ("bottom_right", '┘'),
    ("horizontal",    '─'),
    ("vertical",     '│'),
    ("cross",    '┼'),
];

pub fn lookup(name: &str) -> Option<char> {
    CHARSET.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
}
