#[macro_use]
extern crate rustful;

//use std::error::Error;

//use rustful::{ Server, Context, Response, TreeRouter };

mod example;
//mod 

fn main() {
    example::smash_bros_server().run().ok();
}
