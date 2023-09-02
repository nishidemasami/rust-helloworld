mod utils;

use num::{BigUint, Num, Zero};
// use rand::{Rng, SeedableRng};
// use rand_xorshift::XorShiftRng;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
// use std::cell::RefCell;
use std::fmt::Display;
// use std::io;
use std::ops::Rem;
// use std::rc::Rc;

fn main() {
    fizzbuzz1();
    fizzbuzz2();
    fizzbuzz3();
    // fizzbuzz4();
}

fn fizzbuzz1() {
    // println!(
    //     "{}",
    //     (1..=20)
    //         .par_bridge()
    //         .map(|x| match ((x % 3).is_zero(), (x % 5).is_zero()) {
    //             (true, true) => "FizzBuzz".to_string(),
    //             (true, _) => "Fizz".to_string(),
    //             (_, true) => "Buzz".to_string(),
    //             _ => x.to_string(),
    //         })
    //         .collect::<String>()
    // );

    // println!(
    // "{}",
    (1..=20)
        // .into_par_iter()
        // .par_bridge()
        // .into_par_iter()
        // .par_iter()
        .into_par_iter()
        // .iter()
        .map(|x| match ((x % 3), (x % 5)) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => x.to_string(),
        })
        // .collect::<Vec<String>>()
        // // );
        // .iter()
        // .collect()
        .for_each(|x| println!("{}", x));
}

fn fizzbuzz2() {
    num_iter::range_inclusive(
        BigUint::from_str_radix("18446744073709551616", 10).unwrap(),
        BigUint::from_str_radix("18446744073709551630", 10).unwrap(),
    )
    .par_bridge()
    .into_par_iter()
    // .par_bridge()
    .inspect(|x| println!("Fizz?:{}", (x % BigUint::from(3u64)).is_zero()))
    .inspect(|x| println!("Buzz?:{}", (x % BigUint::from(5u64)).is_zero()))
    .map(|x| {
        match (
            (&x % BigUint::from(3u64)).is_zero(),
            (&x % BigUint::from(5u64)).is_zero(),
        ) {
            (true, true) => "FizzBuzz".to_string(),
            (true, _) => "Fizz".to_string(),
            (_, true) => "Buzz".to_string(),
            _ => x.to_string(),
        }
    })
    // .collect()
    .for_each(|x| println!("{}", x));
}

fn fizzbuzz3() {
    num_iter::range_inclusive(
        BigUint::from_str_radix("340282366920938463463374607431768211456", 10).unwrap(),
        BigUint::from_str_radix("340282366920938463463374607431768211470", 10).unwrap(),
    )
    // .par_bridge()
    // .par_bridge()
    // .into_iter()
    // .par_bridge()
    // .par_bridge()
    // .into_par_iter()
    // .par_bridge()
    // .into_par_iter()
    // .par_bridge()
    // .collect_into(collection)
    // .inspect(|x| println!("Is {} Fizz?:{}", x, (x % BigUint::from(3u32)).is_zero()))
    // .inspect(|x| println!("Is {} Buzz?:{}", x, (x % BigUint::from(5u32)).is_zero()))
    .inspect(println_closure)
    .map(fizzbuzz12)
    // .inspect(print_closure)
    // .collect::<Vec<String>>()
    // .iter()
    // .collect_into()
    // .collect()
    .for_each(|x| println!("{}", x));
}

fn println_closure<T: Display>(text: &T) {
    println!("{}", text);
}

pub fn fizzbuzz10(item: &impl ToString) -> String {
    item.to_string()
}

pub fn fizzbuzz11(item: impl ToString) -> impl ToString {
    item
}

pub fn fizzbuzz12<'a, T, U: Zero>(number: T) -> Box<dyn 'a + Display>
where
    T: 'a,
    // for<'b> &'b T: 'a,
    T: From<u32>,
    T: Zero,
    T: Display,
    for<'b> &'b T: Rem<T, Output = U>,
    // &'a T: Rem<Output = U>,
    // V: Rem<T, Output = U>
{
    // let fizzable = (&number % &T::from(3)).is_zero();
    // let fuzzable = (&number % &T::from(5)).is_zero();
    // if fizzable && fuzzable {
    //     return Box::new("FizzBuzz");
    // } else if fizzable {
    //     return Box::new("Fizz");
    // } else if fuzzable {
    //     return Box::new("Buzz");
    // }
    // Box::new(number)
    // match (
    //     (&number % &T::from(3)).is_zero(),
    //     (&number % &T::from(5)).is_zero(),
    // ) {
    //     (true, true) => "FizzBuzz",
    //     (true, _) => "Fizz",
    //     (_, true) => "Buzz",
    //     _ => number,
    // }
    match (
        (&number % T::from(3)).is_zero(),
        (&number % T::from(5)).is_zero(),
    ) {
        (true, true) => Box::new("FizzBuzz"),
        (true, _) => Box::new("Fizz"),
        (_, true) => Box::new("Buzz"),
        _ => Box::new(number),
    }
}

// pub fn fizzbuzz13<'a, T>(number: T) -> impl 'a + Display
// where
//     T: 'a,
//     T: From<u32>,
//     T: Zero,
//     T: Display,
//     for<'b> &'b T: Rem<Output = T>,
// {
//     // let fizzable = (&number % &T::from(3)).is_zero();
//     // let fuzzable = (&number % &T::from(5)).is_zero();
//     // if fizzable && fuzzable {
//     //     return Box::new("FizzBuzz");
//     // } else if fizzable {
//     //     return Box::new("Fizz");
//     // } else if fuzzable {
//     //     return Box::new("Buzz");
//     // }
//     // Box::new(number)
//     // match (
//     //     (&number % &T::from(3)).is_zero(),
//     //     (&number % &T::from(5)).is_zero(),
//     // ) {
//     //     (true, true) => "FizzBuzz",
//     //     (true, _) => "Fizz",
//     //     (_, true) => "Buzz",
//     //     _ => number,
//     // }
//     match (
//         (&number % &T::from(3)).is_zero(),
//         (&number % &T::from(5)).is_zero(),
//     ) {
//         (true, true) => Box::new("FizzBuzz"),
//         (true, _) => Box::new("Fizz"),
//         (_, true) => Box::new("Buzz"),
//         _ => Box::new(number),
//     }
// }

pub fn fizzbuzz<T>(number: T) -> String
where
    T: From<u32>,
    T: Zero,
    T: ToString,
    for<'fizzbuzz> &'fizzbuzz T: Rem<Output = T>,
{
    match (
        (&number % &T::from(3)).is_zero(),
        (&number % &T::from(5)).is_zero(),
    ) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => number.to_string(),
    }
}
// fn fizzbuzz_closure<T>(number: T) -> impl FnOnce() -> String
// where
//     T: From<u32>,
//     T: Zero,
//     T: ToString,
//     for<'fizzbuzz> &'fizzbuzz T: Rem<Output = T>,
// {
//     move || match (
//         (&number % &T::from(3)).is_zero(),
//         (&number % &T::from(5)).is_zero(),
//     ) {
//         (true, true) => "FizzBuzz".to_string(),
//         (true, _) => "Fizz".to_string(),
//         (_, true) => "Buzz".to_string(),
//         _ => number.to_string(),
//     }
// }

// struct Closure<T> {
//     // ここでは例として `&Data` 型で持っているが、ケースによっては `&mut Data` や `Data` になる
//     x: T,
// }

// impl FnOnce() for Closure<T> {
//     #![feature(unboxed_closures)]
//     fn call_once(self) {
//         match (
//             (&number % &T::from(3)).is_zero(),
//             (&number % &T::from(5)).is_zero(),
//         ) {
//             (true, true) => "FizzBuzz".to_string(),
//             (true, _) => "Fizz".to_string(),
//             (_, true) => "Buzz".to_string(),
//             _ => number.to_string(),
//         }
//     }

//     type Output;
// }

// fn fizzbuzz4() {
//     num_iter::range_inclusive(
//         BigUint::from_str_radix("18446744073709551616", 10).unwrap(),
//         BigUint::from_str_radix("18446744073709551630", 10).unwrap(),
//     )
//     .par_bridge()
//     .map(fizzbuzz)
//     .for_each(|x| println!("{}", x));
// }

#[test]
fn fizz_buzz_test() {
    use num::{BigInt, Num};

    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&1u32).to_string(),
        "1"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&3u32).to_string(),
        "Fizz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&5u32).to_string(),
        "Buzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&15u32).to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&18446744073709551601u64).to_string(),
        "18446744073709551601"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&18446744073709551603u64).to_string(),
        "Fizz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&18446744073709551605u64).to_string(),
        "Buzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&18446744073709551615u64).to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&BigInt::from_str_radix("1", 10).unwrap())
            .to_string(),
        "1"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&BigInt::from_str_radix("3", 10).unwrap())
            .to_string(),
        "Fizz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&BigInt::from_str_radix("5", 10).unwrap())
            .to_string(),
        "Buzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(&BigInt::from_str_radix("15", 10).unwrap())
            .to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("18446744073709551616", 10).unwrap()
        )
        .to_string(),
        "18446744073709551616"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("18446744073709551618", 10).unwrap()
        )
        .to_string(),
        "Fizz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("18446744073709551620", 10).unwrap()
        )
        .to_string(),
        "Buzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("18446744073709551630", 10).unwrap()
        )
        .to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("340282366920938463463374607431768211456", 10).unwrap()
        )
        .to_string(),
        "340282366920938463463374607431768211456"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("340282366920938463463374607431768211458", 10).unwrap()
        )
        .to_string(),
        "Fizz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("340282366920938463463374607431768211460", 10).unwrap()
        )
        .to_string(),
        "Buzz"
    );
    assert_eq!(
        Into::<utils::fizzbuzz::FizzBuzz>::into(
            &BigInt::from_str_radix("340282366920938463463374607431768211470", 10).unwrap()
        )
        .to_string(),
        "FizzBuzz"
    );
}
