use actix_web::{HttpRequest, HttpResponse, Error};
use futures::{future, Future};

pub enum CustomError {
    One,
    Two,
    Three
}

fn build_error_response(e: CustomError) -> HttpResponse {
    match e {
        CustomError::One => {
            HttpResponse::BadRequest().body("err one")
        },
        CustomError::Two => {
            HttpResponse::BadRequest().body("err two")
        },
        CustomError::Three => {
            HttpResponse::BadRequest().body("err three")
        }
    }
}

fn three(num: usize, msg: String) -> Box<Future<Item=String, Error=CustomError>> {
    println!("{}", msg);
    if num == 3 {
        Box::new(future::err(CustomError::Three))
    } else {
        Box::new(future::ok(String::from("finished three")))
    }
}


fn two(num: usize, msg: String) -> Box<Future<Item=String, Error=CustomError>> {
    println!("{}", msg);
    if num == 2 {
        Box::new(future::err(CustomError::Two))
    } else {
        Box::new(future::ok(String::from("finished two")))
    }
}

fn one(num: usize, ) -> Box<Future<Item=String, Error=CustomError>> {
    if num == 1 {
        Box::new(future::err(CustomError::One))
    } else {
        Box::new(future::ok(String::from("finished one")))
    }
}

pub fn errors_endpoint_entry(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    println!("foo");
    let num = req.match_info().get("num").unwrap().parse::<usize>().unwrap();

    let chain = one(num)
        .and_then(move |msg| { two(num, msg) })
        .and_then(move |msg| { three(num, msg) })
        .then(move |r| {
            match r {
                Ok(msg) => {
                    println!("{}", msg);
                    return future::ok(HttpResponse::Ok().finish())
                },
                Err(e) => {
                    return future::ok(build_error_response(e))
                }
            }
        });

    Box::new(chain)
}