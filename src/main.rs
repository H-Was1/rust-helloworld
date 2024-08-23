fn main() {
    let mut a = 4;

    //  loop loop

    'nloop: loop {
        println!("{}", a);
        a -= 1;
        if a == 0 {
            break;
        }
    }

    // let loop

    let x = loop {
        println!("{}", a);

        a += 1;
        if a == 10 {
            break a;
        }
    };

    // while loop

    while a != 0 {
        println!("{}", a);
        a -= 1;
    }

    // for loop
    // println!("{}", a);

}
