#[macro_use]
extern crate serde_derive;

mod db;

use db::{get_row};

fn main() {
    println!("Hello, world!");
    get_row();
}
