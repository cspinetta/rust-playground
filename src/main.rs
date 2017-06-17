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

//mod test_requests;
mod structs_examples;
mod enum_examples;

fn main() {
    println!("Hello, world!");

    structs_examples::test();
    enum_examples::test();
//    play_guess_game();
//    generate_request();
//    generate_request_from_threads();

//    start_server("127.0.0.1:8080", "/tmp/data-rs/");
}

fn play_guess_game() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

fn generate_request() {
    let client = create_client();

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

    let client = Arc::new(create_client());
    let clone1 = client.clone();
    let clone2 = client.clone();
    let handle1 = thread::spawn(move || {
        let mut response = perform_request(&clone1, "https://google.com/");
        show_response(&mut response);
    });
    let handle2 = thread::spawn(move || {
        let mut response = perform_request(&clone2, "https://reddit.com/");
        show_response(&mut response);
    });
    handle1.join();
    handle2.join();

}

fn perform_request(client: &Client, url: &str) -> Response {
    println!("{}", url);
    let safety_url = Url::parse(&url).unwrap();
    println!("{:?}", safety_url);
    let res = client.get(safety_url).send().expect(&format!("Going to {:?}", url));

    assert_eq!(res.status, hyper::Ok);

    res
}

fn create_client() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    return client
}

fn show_response(response: &mut Response) {
    let mut content = String::new();
    match response.read_to_string(&mut content) {
        Ok(c) => println!("Response from {}", c),
        Err(_) => println!("Failed trying to parse response body as string")
    }
}
