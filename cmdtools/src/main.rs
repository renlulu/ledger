extern crate crypto;

use crypto::sha256;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "sha256" => {
            println!("{}", sha256(&args[1]));
        }
        _ => {
            println!("{:?}", &args)
        }
    }
}
