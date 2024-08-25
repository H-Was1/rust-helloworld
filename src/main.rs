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

    modules::matcher::coin_matcher("Penny");
}
