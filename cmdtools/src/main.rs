extern crate crypto;

use crypto::sha256;
use crypto::double_sha256;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "sha256" => {
            println!("{}", sha256(&args[2]));
        }
        "double_sha256" => {
            println!("{}",double_sha256(&args[2]));
        }
        _ => {
            println!("{:?}", &args)
        }
    }
}
