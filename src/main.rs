use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error");
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut less_than_five = Vec::new();
    for x in 0..10 {
        if x < 5 {
            less_than_five.push(x)
        }
    }
    println!("{}", less_than_five[4]);
    println!("{}", less_than_five.len());

    let ez: Vec<i32> = (0..10).filter(|x| *x < 5).collect();
    println!("{:?}", ez);

    println!("The secret_number is: {secret_number}");
    /*  loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    } */

    let mut t: Vec<usize> = Vec::new();
    let u = [2, 5, 3, 1, 9];
    for i in 0..u.len() {
        for j in 1..u.len() {
            if u[i] < u[j] {
                t.push(i);
            } else {
                t.push(j);
            }
        }
    }

    println!("{:?}", t);
}
