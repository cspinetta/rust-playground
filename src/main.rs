extern crate hyper;
extern crate hyper_native_tls;

use std::io;
use std::sync::Arc;
use std::thread;
use hyper::client::{Client, Request, Response};
use std::io::Read;
use hyper::Url;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;


fn main() {
    println!("Hello, world!");

//    play_guess_game();
    generate_request();
//    generate_request_from_threads();
}

fn play_guess_game() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

fn generate_request() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let plain_url = "http://reddit.com/";

    let url = Url::parse(&plain_url).unwrap();
    println!("{:?}", url);

    let mut res = client.get(url).send().expect("Going to reddit.com");
    assert_eq!(res.status, hyper::Ok);


    let mut s = String::new();
    res.read_to_string(&mut s).expect("Response of google.com expected");

    println!("Response: {}", s)
}

fn generate_request_from_threads() {

    let client = Arc::new(Client::new());
    let clone1 = client.clone();
    let clone2 = client.clone();
    thread::spawn(move || {
        let mut res = clone1.get("http://google.com").send().unwrap();
        assert_eq!(res.status, hyper::BadRequest);
    });
    thread::spawn(move || {
        clone2.post("http://example.domain/post").body("foo=bar").send().unwrap();
    });

}
