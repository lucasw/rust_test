/**
 * Copyright (c) 2020 Lucas Walter
 * September 2020
 *
 * Launch a web server.
 * Have end points launch canned processes- like roslaunch
 * Then monitor and optionally stop those processes through the endpoints
 * Also have a main status page that exposes all the end points,
 * lists all the running processes.
 *
 * rocket is out until it can use stable rust instead of nightly,
 * which may be soon.
 *
 * Try rouille 3.0.0
 * https://crates.io/crates/rouille
 */

#[macro_use]
extern crate rouille;

// use std::io;
// use std::env;
use std::process::Command;
// use rouille::cgi::CgiRun;

fn main() {
    println!("Starting server");

    // https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    // https://github.com/tomaka/rouille/blob/master/examples/git-http-backend.rs
    rouille::start_server("localhost:8700", move |request| {
        router!(request,
            (GET) (/) => {
                // If the request's URL is `/hello/world`, we jump here.
                println!("hello world");

                // Builds a `Response` object that contains the "hello world" text.
                rouille::Response::html("<br><br><br><br><b>hello world</b> test
                                        <a href=\"/start\">start</a>")
            },
            (GET) (/start) => {
                // If the request's URL is `/hello/world`, we jump here.
                println!("start");

                // TODO(lucasw) execute a command, store the handle/pid somewhere

                let page: &str = "<br><br><br><br><b>Start Page</b><br>
                                 <a href=\"/\">main</a>";
                // Builds a `Response` object that contains the "hello world" text.
                rouille::Response::html(page)
            },
            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
        /*
        rouille::log(&request, io::stdout(), || {

        })
        */
    });
}
