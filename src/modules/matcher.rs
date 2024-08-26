pub fn matcher(n: i8) {
    match n {
        1 | 2 | 3 => println!("Small"),
        4 | 5 | 6 => println!("Medium"),
        7 | 8 | 9 => println!("Large"),
        _ => println!("Invalid input"),
    }
}

enum COINS {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
pub fn coin_matcher(coin: &str) {
    let go = if coin == "Penny" {
        COINS::Penny
    } else if coin == "Dime" {
        COINS::Dime
    } else if coin == "Quarter" {
        COINS::Quarter
    } else {
        COINS::Nickel
    };
    match go {
        COINS::Dime => println!("this is DIME"),
        COINS::Nickel => println!("this is Nickel"),
        COINS::Penny => println!("this is Penny"),
        COINS::Quarter => println!("this is Quarter"),
        _ => println!("Invalid coin"),
    }
}
