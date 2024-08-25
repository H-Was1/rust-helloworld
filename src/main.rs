mod modules; // Declare the modules folder
fn main() {
    // import a1 and a2 from ./a1.rs

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
    // concat both type &str and creat full name variable
    let first_name: &str = modules::names::a1();
    let last_name = modules::names::a2();
    let full_name = format!("{} {}", first_name, last_name);
    println!("{}", full_name);
}
