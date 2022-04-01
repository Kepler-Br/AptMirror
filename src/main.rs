#![allow(dead_code, unused_variables)]
#![allow(clippy::needless_return)]

use std::fs;
use crate::parsers::parse_in_release;

mod parsers;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let in_release_content = fs::read_to_string("./InRelease")
        .expect("InRelease not exists");
    let parsed = parse_in_release(&in_release_content)
        .expect("Not parsed");

    return Result::Ok(());
}
