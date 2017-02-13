#![feature(use_extern_macros)]

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;

extern crate rustc_serialize;
use rustc_serialize::json;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use serde_json::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountDataArchive {
    id: String,
    network: String,
    network_username: String,
    follower_count: String,
    correlation_id: String,
    archive_id: String,
    type_id: String,
    created_at: String,
    updated_at: String,
    deleted: String,
}

fn main() {
    let object = AccountDataArchive {
        id: "1".to_string(),
        network: "twitter".to_string(),
        network_username: "bobdog".to_string(),
        follower_count: "1000".to_string(),
        correlation_id: "1x45678ILK".to_string(),
        archive_id: "5".to_string(),
        type_id: "2".to_string(),
        created_at: "12345678".to_string(),
        updated_at: "09876542".to_string(),
        deleted: "0".to_string(),
    };

    let encoded = json::encode(&object).unwrap();

    println!("Encoded!!: {:?}", encoded );
    // let john = json!({
    //   "name": "John Doe",
    //   "age": 43,
    //   "phones": [
    //     "+44 1234567",
    //     "+44 2345678"
    //   ]
    // });

}
// .cmp() can be called on anything that can be compared
// println!("Guess the number!");
//
// let secret_number = rand::thread_rng().gen_range(1, 101);
//
// println!("The secret number is: {}", secret_number);
//
// loop {
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin().read_line(&mut guess)
//         .expect("failed to read line");
//
//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue,
//     };
//
//     println!("You guessed: {}", guess);
//
//     match guess.cmp(&secret_number) {
//         Ordering::Less    => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal   => {
//             println!("You win!");
//             break;
//         }
//     }
// }
