use num::{BigUint, Num, Zero};
// use rand::{Rng, SeedableRng};
// use rand_xorshift::XorShiftRng;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
// use std::cell::RefCell;
use std::fmt::Display;
// use std::io;
use std::ops::Rem;
// use std::rc::Rc;

pub fn fizzbuzz<'a, T: 'a + Zero, U: 'a + From<u32> + Zero + Display>(
    number: &'a U,
) -> Box<dyn 'a + Display>
where
    &'a U: Rem<U, Output = T>,
{
    match (
        (number % U::from(3)).is_zero(),
        (number % U::from(5)).is_zero(),
    ) {
        (true, true) => Box::new("FizzBuzz"),
        (true, _) => Box::new("Fizz"),
        (_, true) => Box::new("Buzz"),
        _ => Box::new(number),
    }
}
