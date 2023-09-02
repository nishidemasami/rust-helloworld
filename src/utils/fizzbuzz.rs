use num::Zero;
use std::fmt;
use std::ops::Rem;

pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(String),
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FizzBuzz::Fizz => write!(f, "Fizz"),
            FizzBuzz::Buzz => write!(f, "Buzz"),
            FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzz::Number(x) => write!(f, "{x}"),
        }
    }
}

impl<T, U> From<&T> for FizzBuzz
where
    T: From<u8>,
    for<'a> &'a T: Rem<T, Output = U> + ToString,
    U: Zero,
{
    fn from(x: &T) -> FizzBuzz {
        match ((x % T::from(3)).is_zero(), (x % T::from(5)).is_zero()) {
            (true, true) => FizzBuzz::FizzBuzz,
            (true, _) => FizzBuzz::Fizz,
            (_, true) => FizzBuzz::Buzz,
            _ => FizzBuzz::Number(x.to_string()),
        }
    }
}
