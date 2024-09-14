mod modules;
fn main() {
    let tweet = String::from("rust bootcamp");
    let ans = first_word(&tweet);
    println!("First word: {}", ans);
}

fn first_word(st: &str) -> &str {
    // split the &str at first space and return
    &st[..st.find(' ').unwrap_or(st.len())]
}
