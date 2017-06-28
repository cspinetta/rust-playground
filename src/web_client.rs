//#![deny(warnings)]
//extern crate futures;
//extern crate hyper;
//extern crate tokio_core;
//
//extern crate pretty_env_logger;
//
//use std::env;
//use std::io::Write;
//
//use self::futures::Future;
//use self::futures::stream::Stream;
//use hyper::server::{Http, Service, Request, Response};
//
//use hyper::client::FutureResponse;
//use hyper::Client;
//use web_server::futures::IntoFuture;
//
//pub fn perform_request(request: Request) -> FutureResponse {
//
//    let mut core = tokio_core::reactor::Core::new().unwrap();
//    let handle = core.handle();
//    let client = Client::new(&handle);
//
//    let work = client.request(request)
//
//    .and_then(|res| {
//        println!("Response: {}", res.status());
//        println!("Headers: \n{}", res.headers());
//
//        res.body().for_each(|chunk| {
//            io::stdout().write_all(&chunk).map_err(From::from)
//        })
//    }).map(|_| {
//        println!("\n\nDone.");
//    });
//
//    core.run(work)
//}
