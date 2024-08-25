pub fn first() -> &'static str {
    let first_name = "Hammad";
    first_name
}

pub fn second() -> &'static str {
    let last_name = "Wasi";
    last_name
}

pub fn converter(s: &str, s2: &str) -> String {
    let full_name = format!("{} {}", s, s2);
    full_name
}
