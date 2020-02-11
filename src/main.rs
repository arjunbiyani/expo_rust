// Trying some hands on rust basics
extern crate diesel;
extern crate dotenv;
mod rust_basics;
mod webservices;
use std::env;


fn main() {
    rust_basics::test::run();
}
