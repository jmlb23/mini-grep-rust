extern crate regex;
extern crate colored;

use std::fs::read_to_string;
use std::path::Path;
use regex::*;
use std::env::args;
use colored::*;

fn main() {
    let args = args().collect::<Vec<String>>();

    let regex = &args[1];
    let text = &args[2];

    let path = Path::new(text);
    match read_to_string(path){
        Ok(content) => {
            let string = content.as_str();

            let regex = Regex::new(regex).expect("Error creating the regex");
            let mapped = string.lines().enumerate().filter(|x| regex.is_match(x.1));

            mapped.for_each(|x| println!("{}: {}",x.0.to_string().white(),x.1.bright_red()))
        },
        Err(_s) => {
            println!("{}",format!("file {} not found", text.red()));
            std::process::exit(1);
        },
    };


}
