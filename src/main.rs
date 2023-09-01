use num::{BigInt, BigUint, Num, One, Zero};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::io;
use std::{cmp::Ordering, ops::Rem};

pub fn fizzbuzz_closure(number: BigUint) -> String {
    match (
        (&number % BigUint::from(3u32)).is_zero(),
        (&number % BigUint::from(5u32)).is_zero(),
    ) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => number.to_string(),
    }
}

fn main() {
    println!("Guess the number!");
    (1..=20)
        .map(|x| match ((x % 3).is_zero(), (x % 5).is_zero()) {
            (true, true) => "FizzBuzz".to_string(),
            (true, _) => "Fizz".to_string(),
            (_, true) => "Buzz".to_string(),
            _ => x.to_string(),
        })
        .for_each(|x| println!("{}", x));

    let clousuredesu = |x: BigUint| match (
        (&x % BigUint::from(3u64)).is_zero(),
        (&x % BigUint::from(5u64)).is_zero(),
    ) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => x.to_string(),
    };

    num_iter::range_inclusive(BigUint::from(1u64), BigUint::from(20u64))
        .map(clousuredesu)
        .for_each(|x| println!("{}", x));

    num_iter::range_inclusive(BigUint::from(1u64), BigUint::from(20u64))
        .map(fizzbuzz_closure)
        .for_each(|x| println!("{}", x));
}

/**
 * guessing_game
 * https://github.com/rust-lang/book
 * https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html
 * Apache-2.0, MIT licenses found
 */
fn main_old() {
    println!("Guess the number!");

    // let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_entropy();
    let secret_number = rng.gen_range(1..101);

    loop {
        println!("secret_number: {}", secret_number);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn gcd<T>(x: T, y: T) -> T
where
    T: Zero,
    for<'x> &'x T: Rem<Output = T>,
{
    if x.is_zero() {
        y
    } else if y.is_zero() {
        x
    } else {
        let r = &x % &y;
        gcd(y, r)
    }
}
