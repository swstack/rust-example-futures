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
    Ok(String::from("one done"))
}

fn two_chain() -> Result<String, String> {
//    let r = three_chain()?;
    Ok(String::from("one done"))
}

fn three_chain() -> Result<String, String> {
    Ok(String::from("three done"))
}


fn main_chain() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain() {
        println!("farts")
    }
}
