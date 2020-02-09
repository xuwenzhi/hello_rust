extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
//use std::io;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let secret_number2 = rand::thread_rng().gen_range(1, 101);
    println!("The secrend_number is {}", secret_number);
    println!("The secrend_number2 is {}", secret_number2);

    let name = "Wenzhi Xu";
    //io::stdin()
    //    .read_line(&mut name)
    //    .expect("Failed to read line.");

    match secret_number.cmp(&secret_number2) {
        Ordering::Less => println!("secret_number < secret_number2"),
        Ordering::Equal => println!("secret_number == secret_number2"),
        Ordering::Greater => println!("secret_number > secret_number2"),
    }
    println!("Your name is {}", name);

    // constant
    println!("Constant");
    const MAX_POINTS: u32 = 100_000;
    println!("Max_Points : {}", MAX_POINTS);

    // bool
    println!("Bool");
    let is_signed: bool = 100 == 100;
    println!("is_signed : {}", is_signed);

    // tuple
    println!("Tuple");
    let tuples: (u32, f64, i8) = (2000, 2.20, -2);
    let (x, y, z) = tuples;
    println!(
        "tuples x:{}, y : {}, z : {}, tuples.2 : {}",
        x, y, z, tuples.2
    );

    // array
    println!("Array");
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sta", "Sun"];
    println!("{:?}", days);
    for day in &days {
        print!("{}, ", day);
    }
    println!();

    // Control Flow
    println!("Control Flow");
    if 1 < 2 {
        println!("1 < 2");
    } else {
        println!("1 > 2");
    }
    let mut counter = 0;
    loop {
        counter = counter + 1;
        if counter == 10 {
            counter = 888;
            break;
        }
    }
    println!("counter : {}", counter);
}
