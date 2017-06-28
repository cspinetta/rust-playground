//#![deny(warnings)]
//
//extern crate futures;
//extern crate hyper;
//extern crate pretty_env_logger;
//
//use self::futures::future::FutureResult;
//
//use hyper::client::FutureResponse;
//
//use hyper::{Get, Post, StatusCode};
//use hyper::header::ContentLength;
//use hyper::server::{Http, Service, Request, Response};
//
////use web_client::perform_request;
//
//static INDEX: &'static [u8] = b"Try POST /echo";
//
//struct Echo;
//
//impl Service for Echo {
//    type Request = Request;
//    type Response = Response;
//    type Error = hyper::Error;
//    type Future = FutureResult<Response, hyper::Error>;
//
//    fn call(&self, req: Request) -> Self::Future {
//
//        match (req.method(), req.path()) {
//            (&Get, "/") | (&Get, "/echo") => {
//
//                // TODO from here!!!!!!!
////                perform_request(req).then(|x| {
////                    match x {
////                        Ok(response) => Ok(response),
////                        Error(e) => Error(e)
////                    }
////                })
//                futures::future::ok(Response::new()
//                    .with_header(ContentLength(INDEX.len() as u64))
//                    .with_body(INDEX))
//            },
//            _ => {
//                futures::future::ok(Response::new()
//                                        .with_status(StatusCode::NotFound))
//            }
//        }
////        futures::future::ok(match (req.method(), req.path()) {
////            (&Get, "/") | (&Get, "/echo") => {
////
////                perform_request(req.uri)
////                Response::new()
////                    .with_header(ContentLength(INDEX.len() as u64))
////                    .with_body(INDEX)
////            },
////            (&Post, "/echo") => {
////                let mut res = Response::new();
////                if let Some(len) = req.headers().get::<ContentLength>() {
////                    res.headers_mut().set(len.clone());
////                }
////                res.with_body(req.body())
////            },
////            _ => {
////                Response::new()
////                    .with_status(StatusCode::NotFound)
////            }
////        })
//    }
//
//}
//
//
//pub fn start() {
//    pretty_env_logger::init().unwrap();
//    let addr = "127.0.0.1:1337".parse().unwrap();
//
//    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
//    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
//    server.run().unwrap();
//}
