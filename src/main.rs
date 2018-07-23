#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate regex;
extern crate http;
extern crate ring;
extern crate data_encoding;
extern crate postgres;
extern crate timeago;
extern crate pulldown_cmark;
extern crate ammonia;
extern crate comrak;
extern crate openssl;

use actix_web::{server,actix::System};

mod api;
mod handler;
mod model;
mod utils;
mod router;

fn main() {
    ::std::env::set_var("RUST_LOG", "ruster=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = System::new("ruster");


    server::new( move || router::app())
        .bind("localhost:8000").unwrap()
        .shutdown_timeout(2)
        .start();

    sys.run();
}
