use futures::{future, Future};

pub fn foo() -> Box<dyn Future<Item=bool, Error=bool>> {
    Box::new(future::err(true))
}

#[cfg(test)]
mod tests {
    use crate::examples::future_chains_2::foo;
    use actix_http_test::TestServer;
    use actix_http::HttpService;
    use actix_web::App;
    use futures::{future, Future};

    #[test]
    fn test_map_error() {
        let mut srv = TestServer::new(|| {
            HttpService::new(App::new())
        });

        let test = foo()
            .map_err(|err| {
                println!("errr");
                err
            })
            .and_then(|response| {
                println!("def not here");
                future::ok(true)
            })
            .then(|result| {
                println!("here: {:?}", result);
                future::ok::<bool, bool>(true)
            });
        let result = srv.block_on(test);
        println!("{:?}", result);
    }
}
