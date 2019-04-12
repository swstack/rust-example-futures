fn one() -> Result<String, String> {
    Ok(String::from("one done"))
}

fn two() -> Result<String, String> {
    Ok(String::from("two done"))
}

fn three() -> Result<String, String> {
    Ok(String::from("three done"))
}

fn main_nested() {
    match one() {
        Ok(msg) => {
            println!("{}", msg);
            match two() {
                Ok(msg) => {
                    println!("{}", msg);
                    match three() {
                        Ok(msg) => {
                            println!("{}", msg);
                        },
                        Err(e) => {
                            println!("three failed")
                        }
                    }
                },
                Err(e) => {
                    println!("two failed")
                }
            }
        },
        Err(e) => {
            println!("one failed")
        }
    }
}

fn main_flat() -> Result<String, String> {
    one()?;
    two()?;
    three()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested() {
        main_nested()
    }

    #[test]
    fn test_flat() {
        match main_flat() {
            Ok(msg) => {
                println!("done")
            }
            Err(e) => {
                println!("err")
            }
        }
    }
}
