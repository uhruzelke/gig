use crate::{gig_lib::{Gig, GigSelectionScope}, ui::Session};
use std::io::{self, BufRead, Read, Stdin, Write};
use std::env;
mod gig_lib;
mod ui;



// main need minimal argument processing code

fn main() {
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    args.pop();
    let mut args =args.iter().map(|a| a.as_str()).collect();
    println!("{}",path);
    println!("{:#?}", args);
    let mut s = Session::initilize(Some(path));
    s.interpret_command(&mut args);
    s.run();
}
