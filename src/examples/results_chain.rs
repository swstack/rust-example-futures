use rand::Rng;

fn random_error() -> bool {
    let num = rand::thread_rng().gen_range(1, 3);
    if num == 2 {
        true
    } else {
        false
    }
}

fn one_chain() -> Result<String, String> {
    if random_error() {
        Err(String::from("one err"))
    } else {
        two_chain()
    }

}

fn two_chain() -> Result<String, String> {
    if random_error() {
        Err(String::from("two err"))
    } else {
        three_chain()
    }
}

fn three_chain() -> Result<String, String> {
    if random_error() {
        Err(String::from("three err"))
    } else {
        Ok(String::from("done"))
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain() {
        main()
    }
}
