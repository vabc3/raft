extern crate raft;

use raft::Key;

fn main() {
    let input = "Hello world!";
    let k = Key::new(input);
    println!("{}", k);
}