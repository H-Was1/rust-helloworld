fn main() {
    let id = 14;
    let username = get_username(&id);
    match &username {
        Some(username) => println!("{}", username),
        None => println!("User not found"),
    }
    if let Some(name) = username {
        println!("The username is {}", name);
    }
}

fn get_username(id: &u32) -> Option<String> {
    let query = format!("SELECT * FROM users WHERE id = {}", &id);
    let result = query_db(&"hello"); 
    result.ok()
}

fn query_db(query: &str) -> Result<String, String> {
    if (query.is_empty()) {
        Err("Query database is empty".to_string())
    } else {
        Ok("Ferris".to_string())
    }
}
