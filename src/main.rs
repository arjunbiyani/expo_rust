// Trying some hands on rust basics
extern crate diesel;
extern crate dotenv;

use std::env;

mod rust_basics/test;
mod structs;
mod vars;
mod strings;
mod tuples;
fn main() {
   test::run();
    //vars::run();
    //strings::run();
    //tuples::run();
    //vectors::run();
    //structs::run();
}
