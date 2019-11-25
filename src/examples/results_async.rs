use rand::Rng;
use std::error::Error;
use futures::prelude::*;
use futures::Future;
use futures::future;

pub enum CustomError {
    One,
    Two,
    Three
}

fn random_error() -> bool {
    let num = rand::thread_rng().gen_range(1, 3);
    if num == 2 {
        true
    } else {
        false
    }
}

fn one() -> Box<Future<Item=String, Error=CustomError>> {
    if random_error() {
        Box::new(future::err(CustomError::One))
    } else {
        Box::new(future::ok(String::from("one done")))
    }
}

fn two() -> Box<Future<Item=String, Error=CustomError>> {
    if random_error() {
        Box::new(future::err(CustomError::Two))
    } else {
        Box::new(future::ok(String::from("two done")))
    }
}

fn three() -> Box<Future<Item=String, Error=CustomError>> {
    if random_error() {
        Box::new(future::err(CustomError::Three))
    } else {
        Box::new(future::ok(String::from("three done")))
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain() {
        one();
    }
}
