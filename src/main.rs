#[macro_use]
extern crate rustful;

//use std::error::Error;

//use rustful::{ Server, Context, Response, TreeRouter };

//mod example;
mod tcp_example;

fn main() {
    tcp_example::run_echo();
}
