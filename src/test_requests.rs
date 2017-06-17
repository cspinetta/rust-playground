//// From: https://blog.guillaume-gomez.fr/articles/2017-02-22+Rust+asynchronous+HTTP+server+with+tokio+and+hyper
//
//extern crate futures;
//extern crate hyper;
//extern crate net2;
//extern crate num_cpus;
//extern crate reader_writer;
//extern crate tokio_core;
//
//use futures::{Future, Stream};
//use hyper::{Delete, Get};
//use hyper::status::StatusCode;
//use hyper::method::Method::{Put};
//use hyper::header::ContentLength;
//use hyper::server::{Http, Service, Request, Response};
//use net2::unix::UnixTcpBuilderExt;
//use tokio_core::reactor::Core;
//use tokio_core::net::TcpListener;
//
//use std::thread;
//use std::path::Path;
//use std::net::SocketAddr;
//use std::sync::Arc;
//use std::io::{Read, Write};
//
//fn splitter(s: &str) -> Vec<&str> {
//    s.split('/').filter(|x| !x.is_empty()).collect()
//}
//
//#[derive(Clone)]
//struct AlphaBravo {
//    rw: reader_writer::ReaderWriter,
//}
//
//impl AlphaBravo {
//    pub fn new<P: AsRef<Path>>(path: &P) -> AlphaBravo {
//        AlphaBravo {
//            rw: reader_writer::ReaderWriter::new(path).expect("ReaderWriter::new failed"),
//        }
//    }
//}
//
//impl Service for AlphaBravo {
//    type Request = Request;
//    type Response = Response;
//    type Error = hyper::Error;
//    type Future = Box<::futures::Future<Item=Self::Response, Error=Self::Error>>;
//
//    // Handle HTTP requests made to the server.
//    fn call(&self, req: Request) -> Self::Future {
//        let path = req.path().to_owned();
//        ::futures::finished(match *req.method() {
//            Get => {
//                let values = splitter(&path);
//                if values.len() != 1 || !self.rw.exists(&values[0]) {
//                    Response::new().with_status(StatusCode::NotFound)
//                } else if let Some(mut file) = self.rw.get_file(&values[0], false) {
//                    let mut out = Vec::new();
//                    if file.read_to_end(&mut out).is_ok() {
//                        Response::new().with_body(out)
//                    } else {
//                        Response::new().with_status(StatusCode::InternalServerError)
//                    }
//                } else {
//                    Response::new().with_status(StatusCode::NotFound)
//                }
//            }
//            Put => {
//                // If we didn't receive a key in the uri, we can do nothing.
//                let values = splitter(&path);
//                if values.len() != 1 {
//                    Response::new().with_status(StatusCode::NoContent)
//                } else if let Some(mut file) = self.rw.get_file(&values[0], true) {
//                    match req.headers().get::<ContentLength>() {
//                        Some(&ContentLength(len)) => {
//                            // If there is no content, there is nothing to do.
//                            if len < 1 {
//                                Response::new().with_status(StatusCode::NotModified)
//                            } else {
//                                // The interesting part is here: for each chunk, we write it into
//                                // the file.
//                                return Box::new(req.body()
//                                    .for_each(move |chunk| {
//                                        if file.write(&*chunk).is_ok() {
//                                            Ok(())
//                                        } else {
//                                            Err(hyper::Error::Status)
//                                        }
//                                    }).then(|r| match r {
//                                    Ok(_) => Ok(Response::new().with_status(StatusCode::Ok)),
//                                    Err(_) => Ok(Response::new().with_status(
//                                        StatusCode::InsufficientStorage)),
//                                }));
//                            }
//                        }
//                        None => Response::new().with_status(StatusCode::NotModified),
//                    }
//                } else {
//                    Response::new().with_status(StatusCode::InternalServerError)
//                }
//            }
//            Delete => {
//                let values = splitter(&path);
//                if values.len() == 1 && self.rw.exists(&values[0]) {
//                    match self.rw.remove(&values[0]) {
//                        Ok(_) => Response::new().with_status(StatusCode::Ok),
//                        Err(_) => Response::new().with_status(StatusCode::InternalServerError),
//                    }
//                } else {
//                    Response::new().with_status(StatusCode::NotFound)
//                }
//            }
//            _ => {
//                Response::new().with_status(StatusCode::NotFound)
//            }
//        }).boxed()
//    }
//}
//
//// Nothing fancy in here, we start an instance of our server with one reactor.
//fn start_server<P: AsRef<Path>>(addr: &str, path: P) {
//    let addr = addr.parse().unwrap();
//    let (listening, server) = Server::standalone(|tokio| {
//        let values = Rc::new(RefCell::new((HashMap::new())));
//        Server::http(&addr, tokio)?.handle(move || Ok(AlphaBravo::new(&path)), tokio)
//    }).unwrap();
//    println!("Listening on http://{}", listening);
//    server.run();
//}
//
////fn main() {
////    start_server("127.0.0.1:8080", "/tmp/data-rs/");
////}
