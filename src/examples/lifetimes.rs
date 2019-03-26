use futures::{Future, future};
use actix_web::{Error, HttpRequest, HttpResponse};

trait Thing {
    fn do_thing(&self) -> Box<Future<Item=String, Error=Error>>;
}

struct ThingImpl1 {}

impl Thing for ThingImpl1 {
    fn do_thing(&self) -> Box<Future<Item=String, Error=Error>> {
        Box::new(future::ok(String::from("thing 1")))
    }
}

struct ThingImpl2 {}

impl Thing for ThingImpl2 {
    fn do_thing(&self) -> Box<Future<Item=String, Error=Error>> {
        Box::new(future::ok(String::from("thing 2")))
    }
}

struct Stuff {
    things: Vec<Box<dyn Thing>>
}

impl Stuff {
    pub fn new() -> Self {
        Stuff {
            things: Vec::new()
        }
    }

    fn add_thing(&mut self, thing: Box<dyn Thing>) {
        self.things.push(thing)
    }

    fn do_all_things(&self) -> Box<Future<Item=Vec<String>, Error=Error>> {
        let mut futures = Vec::new();
        self.things.iter().for_each(|thing| {
            futures.push(thing.do_thing())
        });

        Box::new(future::join_all(futures))
    }
}

pub fn lifetime_endpoint_entry(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let thing1 = ThingImpl1 {};
    let thing2 = ThingImpl2 {};
    let mut stuff = Stuff::new();
    stuff.add_thing(Box::new(thing1));
    stuff.add_thing(Box::new(thing2));
    Box::new(
        stuff.do_all_things()
            .then(|results| {
                match results {
                    Ok(items) => {
                        println!("{:?}", items);
                        return future::ok(HttpResponse::Ok().finish())
                    },
                    Err(e) => {
                        return future::ok(HttpResponse::BadRequest().finish())
                    }
                }
            })
    )
}
