use std::thread;
use std::time::Duration;
use std::env;
use std::fs;
use std::sync::mpsc;

// is this a module or a crate?
// what is the difference between modules and crates?
use std::net::{TcpListener, TcpStream};

// exposes traits to read from and write to the stream
use std::io::prelude::*;

// look at how you could possibly improve the throughput of a web server
// single-thread async I/O model just like Node.js
// fork/join model
// write the client interface first, it'll help guide your API design.
// APIs should be client-centric
// spawn a thread for each request? (whew, so ghetto)


fn main() {

    // using move closures with threads
    // let's use data from one thread in another thread
    // what does unwrap do/mean?
    // there's no error handling here?
    // custom made struct
    // let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established!");
        // am I passing it the stream or a reference to the stream?
        // passing and executing an anonymous function like that!
            handle_connections(stream);
    }
}

fn handle_connections(mut stream: TcpStream) {
    // buffer is declared on the stack (fixed size)
    // it's large enough to hold request data
    // buffer size could be made dynamic and its data could be stored on the heap but this will add more complexity because it'd introduce buffer management
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // handle HTTP requests based on type
    // only works with HTTP v1 though?
    enum HttpMethod {
        DELETE(String),
        GET(String),
        POST(String),
        PUT(String)
    }

    // these are hardcoded values
    //let delete = HttpMethod::DELETE(String.from(b"DELETE / HTTP/1.1\r\n"));
    // the b at the beginning indicates a byte string!!!
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    //let post = HttpMethod::POST(String.from(b"POST / HTTP/1.1\r\n"));
    //let put = HttpMethod::PUT(String.from(b"PUT / HTTP/1.1\r\n"));

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "error.html")
    };
    // converts bytes into a string in order to print them out
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // create http response
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // write response into stream
    // request and responses are exchanged as bytes
    stream.write(response.as_bytes()).unwrap();

    // not sure what this line does
    // waits til the response is written before continuing the program
    stream.flush().unwrap();
}

// should expect a stream as a parameter to operate on
fn handle_get() {

}

fn handle_post() {

}

fn handle_put() {

}

fn handle_delete() {

}


fn run_threads () {
    // using a join handle over here
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    api::connection();
    // not very sure where box is made available
    let b = Box::new(5);
    println!("b = {}", b);

    // use join handle
    // spawned thread is supposed to execute 10 times
    handle.join().unwrap_err();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub mod api {
    pub(crate) fn connection () {
        println!("Connecting to NeyDB")
    }
}
