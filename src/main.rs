use std::fmt::Error;

mod modules;
fn main() {
    // let mut a = 4;

    // //  loop loop

    // 'nloop: loop {
    //     println!("{}", a);
    //     a -= 1;
    //     if a == 0 {
    //         break;
    //     }
    // }

    // // let loop

    // let x = loop {
    //     println!("{}", a);

    //     a += 1;
    //     if a == 10 {
    //         break a;
    //     }
    // };

    // // while loop

    // while a != 0 {
    //     println!("{}", a);
    //     a -= 1;
    // }

    // // for loop
    // // println!("{}", a);
    // for i in 0..10 {
    //     println!("{}", i);
    // }

    // // match statement

    // let b = 5;
    // match b {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("other"),
    // }

    // struct User {
    //     age: i8,
    //     email: String,
    //     rating: f32,
    // }

    // let dev = User {
    //     age: 32,
    //     email: String::from("john.doe@example.com"),
    //     rating: 1.90,
    // };
    // let name = String::from("john");
    // printer(&name);
    // println!("{}", name);

    // println!("Age: {}", dev.age);
    // println!("Email: {}", dev.email);

    let result = match divider(32, 16) {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {}", err);
            0
        }
    };
    println!("division: {}", result);

    let str_data:String = String::from("Rust");
}

fn divider(a: u32, b: u32) -> Result<u32, String> {
    if a == 0 || b == 0 {
        return Err(String::from("Cannot divide a number by zero"));
    };
    Ok((a % b))
}
